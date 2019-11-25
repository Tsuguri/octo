use super::ast;
use super::ir::{Address, ConstantValue, Operation, PipelineIR};

use super::code::{Code, PhiCollection};

pub fn emit(ast: ast::Pipeline) -> PipelineIR {
    let mut code = Code::new();
    let mut arguments = vec![];

    for arg in ast.arguments.iter().enumerate() {
        let addr = code.push(Operation::Arg(arg.0));
        code.store(&arg.1.identifier.val, addr, false);
        arguments.push((arg.1.typ, arg.1.identifier.val.clone()));
    }
    println!("inputs: {}", ast.arguments.len());
    println!("inputs2: {}", arguments.len());
    println!("outputs: {}", ast.results.len());

    emit_block(ast.block, &mut code);

    let mut output = code.finish();
    output.inputs = arguments;
    output.outputs = ast.results.iter().map(|x| x.val).collect();
    println!("outputs2: {}", output.outputs.len());
    output
}

fn emit_block(block: ast::Block, code: &mut Code) {
    for statement in block.statements {
        emit_statement(statement, code);
    }
}

fn emit_statement(statement: ast::Statement, code: &mut Code) {
    match statement {
        ast::Statement::Expression(exp) => {
            emit_expression(*exp, code);
        }
        ast::Statement::Return(exp) => {
            let ret_add = emit_expression(*exp, code);
            code.exit(ret_add);
        }
        ast::Statement::Assignment(var, exp, create) => {
            let addr = emit_expression(*exp, code);
            let addr = code.push(Operation::Store(addr));
            code.store(&var.identifier.val, addr, create);
        }
        ast::Statement::For(stat, exp1, exp2, block) => {
            // initialization statement
            let _ = emit_statement(*stat, code);

            let post_init_label = code.last_label();

            let loop_def_label = code.new_label();
            let condition_label = code.new_label();
            let continue_label = code.new_label();
            let content_label = code.new_label();
            let end_label = code.new_label();

            code.push(Operation::Jump(loop_def_label));
            code.push_with_label(Operation::Label, loop_def_label);
            let phi_insert_index = code.code_size();
            // phi nodes go here?
            code.push(Operation::LoopMerge(continue_label, end_label));
            code.push(Operation::Jump(condition_label));

            // eval condition
            let old_phi = code.observe_assignments();
            let before_code_size = code.code_size();

            code.push_with_label(Operation::Label, condition_label);
            let cond = emit_expression(*exp1, code);
            code.push(Operation::JumpIfElse(cond, content_label, end_label));

            code.push_with_label(Operation::Label, content_label);

            // do loop stuff
            emit_block(block, code);
            code.push(Operation::Jump(continue_label));
            code.push_with_label(Operation::Label, continue_label);
            // increment
            let _ = emit_statement(*exp2, code);

            code.push(Operation::Jump(loop_def_label));

            let phi_assignments = code.finish_observing(old_phi);
            assert!(phi_assignments.is_some());

            let after_code_size = code.code_size();
            code.push_with_label(Operation::Label, end_label);

            // these phi nodes shall go into loop merge block
            for (id, phi) in phi_assignments.unwrap().iter().enumerate() {
                let mut rec = *phi.1;

                rec.label = continue_label;
                rec.old_label = post_init_label;

                let address = code.insert_at(Operation::Phi(rec), phi_insert_index);
                code.store(&phi.0, address, false);
                //old, new
                code.replace_label(
                    (before_code_size + id)..(after_code_size + id),
                    phi.1.old,
                    address,
                );
            }
        }
        ast::Statement::IfElse(condition, true_block, false_block) => {
            let cond = emit_expression(*condition, code);

            let if_label = code.new_label();
            let else_label = code.new_label();
            let end_label = code.new_label();

            let pre_cond_label = code.last_label();
            let label2 = if false_block.is_some() {
                else_label
            } else {
                end_label
            };
            // jump to first block
            code.push(Operation::JumpIfElse(cond, if_label, label2));

            let old_phi = code.observe_assignments();
            code.push_with_label(Operation::Label, if_label);
            emit_block(true_block, code);
            let true_assignments = code.finish_observing(old_phi).unwrap();
            let post_true_label = code.last_label();
            code.push(Operation::Jump(end_label));

            let mut false_assignments = None;
            let mut post_false_label = pre_cond_label;
            if let Some(bl) = false_block {
                let old_phi = code.observe_assignments();
                code.push_with_label(Operation::Label, else_label);
                emit_block(bl, code);
                false_assignments = code.finish_observing(old_phi);
                post_false_label = code.last_label();
                code.push(Operation::Jump(end_label));
            }
            let post_false_label = post_false_label;

            code.push_with_label(Operation::Label, end_label);

            // emit phi instructions
            for phi in select_phi_operations(
                true_assignments,
                false_assignments,
                post_true_label,
                post_false_label,
            ) {
                let address = code.push(Operation::Phi(phi.1));
                code.store(&phi.0, address, false);
            }
        }
    }
}

fn select_phi_operations(
    true_block: PhiCollection,
    false_block: Option<PhiCollection>,
    true_label: Address,
    false_label: Address,
) -> PhiCollection {
    let mut results = true_block;

    for (name, item) in results.iter_mut() {
        item.label = true_label;
        item.old_label = false_label;
    }

    match false_block {
        None => {}
        Some(x) => {
            for (key, mut record) in x {
                record.label = false_label;
                record.old_label = true_label;
                match results.get(&key) {
                    None => {
                        results.insert(key, record);
                    }
                    Some(&true_phi) => {
                        record.old = true_phi.new;
                        results.insert(key, record);
                    }
                }
            }
        }
    }
    results
}

fn emit_expression(exp: ast::Expression, code: &mut Code) -> Address {
    use ast::Expression::*;
    match exp {
        Variable(var) => code.get(&var.identifier.val),
        Literal(lit) => match lit {
            ast::Literal::Int(val) => code.store_constant(ConstantValue::Int(val.val)),
            ast::Literal::Float(val) => code.store_constant(ConstantValue::Float(val.val)),
        },
        Negation(exp) => {
            let exp_address = emit_expression(*exp, code);
            code.push(Operation::Neg(exp_address))
        }
        Mul(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Mul(left_address, right_address))
        }
        Div(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Div(left_address, right_address))
        }
        Add(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Add(left_address, right_address))
        }
        Sub(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Sub(left_address, right_address))
        }
        Less(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Less(left_address, right_address))
        }
        MoreEqual(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Less(right_address, left_address))
        }
        LessEqual(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::LessEq(left_address, right_address))
        }
        More(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::LessEq(right_address, left_address))
        }
        Equals(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Eq(left_address, right_address))
        }
        NotEquals(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Neq(left_address, right_address))
        }
        And(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::And(left_address, right_address))
        }
        Or(exp_left, exp_right) => {
            let left_address = emit_expression(*exp_left, code);
            let right_address = emit_expression(*exp_right, code);
            code.push(Operation::Or(left_address, right_address))
        }
        Shift(shifted, shift_by) => {
            let left_address = emit_expression(*shifted, code);
            let right_address = emit_expression(*shift_by, code);
            let left_synced = code.synchronize(left_address);
            code.push(Operation::Shift(left_synced, right_address))
        }
        Scale(_scaled, _scale_by) => 0,
        Invocation(..) => {
            0
        }
    }
}
