// auto-generated: "lalrpop 0.15.2"
// sha256: e65378b99772f5694fea85a8e3d2ba68c018e0831c73ac60e0bd2c1b3514fee8
use std::str::FromStr;
use lexer;
use ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Block {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 19, 0, 20, 0, 0, 21,
        // State 3
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 19, 0, 20, 0, 0, 23,
        // State 4
        0, 24, 0, -51, 0, 25, 0, 26, 0, 0, 0, -51, -51, -51, 0, 27, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 7
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, -44, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 8
        0, -16, 0, -16, 30, -16, 0, -16, 0, 31, 0, -16, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 9
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, -39, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 10
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, -43, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 11
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 32, 33, 0, 0, 34, 35, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 14
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 15
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, -37, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 16
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, -45, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 17
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, -36, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, -38, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 24
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 25
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 26
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 27
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 28
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 29
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 30
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 31
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 32
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 33
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 34
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 35
        -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, 0, -9, -9, 0, -9, 0, 0, -9,
        // State 36
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, -42, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, 0, -10, -10, 0, -10, 0, 0, -10,
        // State 40
        0, 0, 0, -30, 0, 25, 0, 26, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 41
        0, -17, 0, -17, 30, -17, 0, -17, 0, 31, 0, -17, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 42
        0, -18, 0, -18, 30, -18, 0, -18, 0, 31, 0, -18, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 43
        0, 0, 0, -29, 0, 25, 0, 26, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 44
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 45
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 46
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, -40, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 47
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, -41, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 48
        0, 0, 0, -52, 0, 25, 0, 26, 0, 0, 0, -52, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 49
        0, 0, 0, -53, 0, 25, 0, 26, 0, 0, 0, -53, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 50
        0, 0, 0, -54, 0, 25, 0, 26, 0, 0, 0, -54, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 51
        0, 0, 0, -55, 0, 25, 0, 26, 0, 0, 0, -55, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 52
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, -46, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
        // State 53
        14, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 17, 0, 18, 0, 0, 20, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -58,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -19,
        // State 21
        0,
        // State 22
        -20,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 4, 0, 0, 0, 5, 0, 0, 0, 0, 6, 7, 0, 0, 8, 9, 10, 11, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 6, 7, 0, 0, 8, 9, 10, 11, 0, 0, 12, 22, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 6, 38, 0, 0, 8, 9, 10, 11, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 42, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 43, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 45, 0, 0, 0, 8, 9, 10, 11, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 46, 0, 0, 0, 8, 9, 10, 11, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 47, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 48, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 8, 9, 10, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 6, 55, 0, 0, 8, 9, 10, 11, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct BlockParser {
        _priv: (),
    }

    impl BlockParser {
        pub fn new() -> BlockParser {
            BlockParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Block, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Block,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                // __Block = Block => ActionFn(3);
                let __sym0 = __pop_Variant11(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(__sym0);
                return Some(Ok(__nt));
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__Block::BlockParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expression {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 1
        0, 16, 0, -51, 0, 17, 0, 18, 0, 0, 0, 0, -51, -51, 0, 19, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 2
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 4
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, 0, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 5
        0, -16, 0, -16, 22, -16, 0, -16, 0, 23, 0, 0, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 6
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, 0, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 7
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, 0, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 0, 0, 26, 27, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 9
        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 10
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 11
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, 0, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 12
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, 0, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 13
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, 0, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 14
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, 0, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 15
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 16
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 17
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 18
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 19
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 20
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 21
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 22
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 23
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 24
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 25
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 26
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 13, 0, 14, 0, 0, 15, 0, 0, 0,
        // State 27
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, 0, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 29
        0, 0, 0, -30, 0, 17, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 30
        0, -17, 0, -17, 22, -17, 0, -17, 0, 23, 0, 0, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 31
        0, -18, 0, -18, 22, -18, 0, -18, 0, 23, 0, 0, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -29, 0, 17, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 34
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 35
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, 0, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 36
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, 0, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 37
        0, 0, 0, -52, 0, 17, 0, 18, 0, 0, 0, 0, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 38
        0, 0, 0, -53, 0, 17, 0, 18, 0, 0, 0, 0, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 39
        0, 0, 0, -54, 0, 17, 0, 18, 0, 0, 0, 0, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -55, 0, 17, 0, 18, 0, 0, 0, 0, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 41
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, 0, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -51,
        // State 2
        -31,
        // State 3
        -59,
        // State 4
        -44,
        // State 5
        -16,
        // State 6
        -39,
        // State 7
        -43,
        // State 8
        -28,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -37,
        // State 12
        -45,
        // State 13
        -36,
        // State 14
        -38,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -42,
        // State 28
        0,
        // State 29
        -30,
        // State 30
        -17,
        // State 31
        -18,
        // State 32
        -29,
        // State 33
        -32,
        // State 34
        -33,
        // State 35
        -40,
        // State 36
        -41,
        // State 37
        -52,
        // State 38
        -53,
        // State 39
        -54,
        // State 40
        -55,
        // State 41
        -46,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 4, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 29, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 31, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 32, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 34, 0, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 35, 0, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 36, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 37, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExpressionParser {
        _priv: (),
    }

    impl ExpressionParser {
        pub fn new() -> ExpressionParser {
            ExpressionParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Expression, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Expression,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                // __Expression = Expression => ActionFn(5);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(__sym0);
                return Some(Ok(__nt));
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__Expression::ExpressionParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Function {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, -21, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -23, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0,
        // State 11
        0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 33, 0, 34, 0, 0, 35,
        // State 16
        0, 0, 0, -25, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 33, 0, 34, 0, 0, 37,
        // State 18
        0, 38, 0, -51, 0, 39, 0, 40, 0, 0, 0, -51, -51, -51, 0, 41, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 19
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0,
        // State 21
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, -44, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 22
        0, -16, 0, -16, 44, -16, 0, -16, 0, 45, 0, -16, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 23
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, -39, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 24
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, -43, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 25
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 46, 47, 0, 0, 48, 49, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 28
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 29
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, -37, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 30
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, -45, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 31
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, -36, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, -38, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 38
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 39
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 40
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 41
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 42
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 43
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 44
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 45
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 46
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 47
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 48
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 49
        -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, 0, -9, -9, 0, -9, 0, 0, -9,
        // State 50
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, -42, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, 0, -10, -10, 0, -10, 0, 0, -10,
        // State 54
        0, 0, 0, -30, 0, 39, 0, 40, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 55
        0, -17, 0, -17, 44, -17, 0, -17, 0, 45, 0, -17, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 56
        0, -18, 0, -18, 44, -18, 0, -18, 0, 45, 0, -18, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 57
        0, 0, 0, -29, 0, 39, 0, 40, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 58
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 59
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 60
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, -40, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 61
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, -41, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 62
        0, 0, 0, -52, 0, 39, 0, 40, 0, 0, 0, -52, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 63
        0, 0, 0, -53, 0, 39, 0, 40, 0, 0, 0, -53, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 64
        0, 0, 0, -54, 0, 39, 0, 40, 0, 0, 0, -54, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 65
        0, 0, 0, -55, 0, 39, 0, 40, 0, 0, 0, -55, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 66
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, -46, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
        // State 67
        28, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 0, 34, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -60,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -34,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        -19,
        // State 35
        0,
        // State 36
        -20,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 18, 0, 0, 0, 19, 0, 0, 0, 0, 20, 21, 0, 0, 22, 23, 24, 25, 0, 0, 26, 27, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 20, 21, 0, 0, 22, 23, 24, 25, 0, 0, 26, 36, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 20, 52, 0, 0, 22, 23, 24, 25, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 56, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 57, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 59, 0, 0, 0, 22, 23, 24, 25, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 60, 0, 0, 0, 22, 23, 24, 25, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 61, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 62, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 24, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 20, 69, 0, 0, 22, 23, 24, 25, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct FunctionParser {
        _priv: (),
    }

    impl FunctionParser {
        pub fn new() -> FunctionParser {
            FunctionParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Function, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Function,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                // __Function = Function => ActionFn(1);
                let __sym0 = __pop_Variant14(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__Function::FunctionParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__GpuFunction {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, -21, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -23, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0,
        // State 11
        0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0,
        // State 15
        0, 0, 0, -25, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -61,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -35,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct GpuFunctionParser {
        _priv: (),
    }

    impl GpuFunctionParser {
        pub fn new() -> GpuFunctionParser {
            GpuFunctionParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::GpuFunction, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::GpuFunction,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                // __GpuFunction = GpuFunction => ActionFn(2);
                let __sym0 = __pop_Variant15(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__GpuFunction::GpuFunctionParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Primitive {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 1
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, 0, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 4
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, 0, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 5
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, 0, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 6
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, 0, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 7
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, 0, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 8
        0, 17, 0, -51, 0, 18, 0, 19, 0, 0, 0, 0, -51, -51, 0, 20, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 11
        0, -16, 0, -16, 24, -16, 0, -16, 0, 25, 0, 0, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 12
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, 0, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 13
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, 0, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 0, 0, 28, 29, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 15
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 16
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 17
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 18
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 19
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 20
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, 0, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
        // State 21
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 22
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 23
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 24
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 25
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 26
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 27
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 28
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 6, 0, 7, 0, 0, 8, 0, 0, 0,
        // State 29
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, 0, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 30
        0, 0, 0, -30, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 31
        0, -17, 0, -17, 24, -17, 0, -17, 0, 25, 0, 0, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 32
        0, -18, 0, -18, 24, -18, 0, -18, 0, 25, 0, 0, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -29, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 34
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 35
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 36
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, 0, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 37
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, 0, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 38
        0, 0, 0, -52, 0, 18, 0, 19, 0, 0, 0, 0, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 39
        0, 0, 0, -53, 0, 18, 0, 19, 0, 0, 0, 0, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -54, 0, 18, 0, 19, 0, 0, 0, 0, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 41
        0, 0, 0, -55, 0, 18, 0, 19, 0, 0, 0, 0, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -44,
        // State 2
        -62,
        // State 3
        0,
        // State 4
        -37,
        // State 5
        -45,
        // State 6
        -36,
        // State 7
        -38,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -46,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 10, 11, 0, 0, 2, 12, 13, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 33, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 35, 0, 0, 0, 2, 12, 13, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 36, 0, 0, 0, 2, 12, 13, 14, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 37, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 38, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct PrimitiveParser {
        _priv: (),
    }

    impl PrimitiveParser {
        pub fn new() -> PrimitiveParser {
            PrimitiveParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Expression, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Expression,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                // __Primitive = Primitive => ActionFn(6);
                let __sym0 = __pop_Variant10(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(__sym0);
                return Some(Ok(__nt));
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__Primitive::PrimitiveParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, -21, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -23, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0,
        // State 16
        0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 38, 0, 39, 0, 0, 40,
        // State 21
        0, 0, 0, -25, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 38, 0, 42, 0, 0, 43,
        // State 23
        0, 44, 0, -51, 0, 45, 0, 46, 0, 0, 0, -51, -51, -51, 0, 47, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 24
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
        // State 26
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, -44, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 27
        0, -16, 0, -16, 50, -16, 0, -16, 0, 51, 0, -16, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 28
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, -39, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 29
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, -43, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 30
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 52, 53, 0, 0, 54, 55, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 33
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 34
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, -37, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 35
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, -45, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 36
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, -36, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -38, 0, 0, -38, -38, 0, -38, 0, -38, 0, -38, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 60,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, -38, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 44
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 45
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 46
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 47
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 48
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 49
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 50
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 51
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 52
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 53
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 54
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 55
        -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, 0, -9, -9, 0, -9, 0, 0, -9,
        // State 56
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, -42, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, -10, 0, -10, -10, 0, -10, 0, 0, -10,
        // State 61
        0, 0, 0, -30, 0, 45, 0, 46, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 62
        0, -17, 0, -17, 50, -17, 0, -17, 0, 51, 0, -17, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 63
        0, -18, 0, -18, 50, -18, 0, -18, 0, 51, 0, -18, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 64
        0, 0, 0, -29, 0, 45, 0, 46, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 65
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 66
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 67
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, -40, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 68
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, -41, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 69
        0, 0, 0, -52, 0, 45, 0, 46, 0, 0, 0, -52, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 70
        0, 0, 0, -53, 0, 45, 0, 46, 0, 0, 0, -53, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 71
        0, 0, 0, -54, 0, 45, 0, 46, 0, 0, 0, -54, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 72
        0, 0, 0, -55, 0, 45, 0, 46, 0, 0, 0, -55, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 73
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, -46, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
        // State 74
        33, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 36, 0, 37, 0, 0, 42, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -47,
        // State 1
        -48,
        // State 2
        -49,
        // State 3
        -50,
        // State 4
        -63,
        // State 5
        -14,
        // State 6
        0,
        // State 7
        -15,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -34,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        -19,
        // State 40
        0,
        // State 41
        0,
        // State 42
        -20,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        -35,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        0,
        // State 65
        0,
        // State 66
        0,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        0,
        // State 71
        0,
        // State 72
        0,
        // State 73
        0,
        // State 74
        0,
        // State 75
        0,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 12, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 23, 0, 0, 0, 24, 0, 0, 0, 0, 25, 26, 0, 0, 27, 28, 29, 30, 0, 0, 31, 32, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 25, 26, 0, 0, 27, 28, 29, 30, 0, 0, 31, 41, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 0, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 25, 58, 0, 0, 27, 28, 29, 30, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 63, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 64, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 66, 0, 0, 0, 27, 28, 29, 30, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 67, 0, 0, 0, 27, 28, 29, 30, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 68, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 0, 69, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 70, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 0, 0, 0, 0, 0, 0, 0, 0, 27, 28, 29, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 25, 76, 0, 0, 27, 28, 29, 30, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 75
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Program, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Program,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant17(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            64 => {
                __reduce64(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce64<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Statement = Statement => ActionFn(4);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 32)
    }
}
pub use self::__parse__Program::ProgramParser;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Statement {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use lexer;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(lexer::Token),
        Variant1(f64),
        Variant2(String),
        Variant3(i64),
        Variant4((ast::Variable, String)),
        Variant5(::std::vec::Vec<(ast::Variable, String)>),
        Variant6(ast::Statement),
        Variant7(::std::vec::Vec<ast::Statement>),
        Variant8(ast::ProgramItem),
        Variant9(::std::vec::Vec<ast::ProgramItem>),
        Variant10(ast::Expression),
        Variant11(ast::Block),
        Variant12(Vec<(ast::Variable, String)>),
        Variant13(::std::option::Option<(ast::Variable, String)>),
        Variant14(ast::Function),
        Variant15(ast::GpuFunction),
        Variant16(ast::Literal),
        Variant17(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 16, 0, 17, 0, 0, 0,
        // State 1
        0, 18, 0, -51, 0, 19, 0, 20, 0, 0, 0, 0, -51, -51, 0, 21, -51, -51, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0,
        // State 2
        0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 4
        0, -44, 0, -44, -44, -44, 0, -44, 0, -44, 0, 0, -44, -44, 0, -44, -44, -44, 0, 0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, 0, 0, 0, 0,
        // State 5
        0, -16, 0, -16, 24, -16, 0, -16, 0, 25, 0, 0, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 6
        0, -39, 0, -39, -39, -39, 0, -39, 0, -39, 0, 0, -39, -39, 0, -39, -39, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, 0, 0, 0, 0,
        // State 7
        0, -43, 0, -43, -43, -43, 0, -43, 0, -43, 0, 0, -43, -43, 0, -43, -43, -43, 0, 0, 0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 0, 0, 28, 29, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 11
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 12
        0, -37, 0, -37, -37, -37, 0, -37, 0, -37, 0, 0, -37, -37, 0, -37, -37, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0, 0,
        // State 13
        0, -45, 0, -45, -45, -45, 0, -45, 0, -45, 0, 0, -45, -45, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, 0,
        // State 14
        0, -36, 0, -36, -36, -36, 0, -36, 0, -36, 0, 0, -36, -36, 0, -36, -36, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -38, 0, -38, -38, -38, 0, -38, 0, -38, 0, 0, -38, -38, 0, -38, -38, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, 0,
        // State 17
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 18
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 19
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 20
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 21
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 22
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 23
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 24
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 25
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 26
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 27
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 28
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 29
        0, -42, 0, -42, -42, -42, 0, -42, 0, -42, 0, 0, -42, -42, 0, -42, -42, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -30, 0, 19, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0,
        // State 33
        0, -17, 0, -17, 24, -17, 0, -17, 0, 25, 0, 0, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 34
        0, -18, 0, -18, 24, -18, 0, -18, 0, 25, 0, 0, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 35
        0, 0, 0, -29, 0, 19, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 36
        0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0,
        // State 37
        0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, -33, 0, 0, 0, 0,
        // State 38
        0, -40, 0, -40, -40, -40, 0, -40, 0, -40, 0, 0, -40, -40, 0, -40, -40, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, -40, 0, 0, 0, 0,
        // State 39
        0, -41, 0, -41, -41, -41, 0, -41, 0, -41, 0, 0, -41, -41, 0, -41, -41, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, -41, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -52, 0, 19, 0, 20, 0, 0, 0, 0, -52, -52, 0, 0, -52, -52, 0, 0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0, 0,
        // State 41
        0, 0, 0, -53, 0, 19, 0, 20, 0, 0, 0, 0, -53, -53, 0, 0, -53, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0, 0,
        // State 42
        0, 0, 0, -54, 0, 19, 0, 20, 0, 0, 0, 0, -54, -54, 0, 0, -54, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, -54, 0, 0, 0, 0,
        // State 43
        0, 0, 0, -55, 0, 19, 0, 20, 0, 0, 0, 0, -55, -55, 0, 0, -55, -55, 0, 0, 0, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, -55, 0, 0, 0, 0,
        // State 44
        0, -46, 0, -46, -46, -46, 0, -46, 0, -46, 0, 0, -46, -46, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, 0,
        // State 45
        11, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 14, 0, 15, 0, 0, 17, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -51,
        // State 2
        -31,
        // State 3
        -56,
        // State 4
        -44,
        // State 5
        -16,
        // State 6
        -39,
        // State 7
        -43,
        // State 8
        -28,
        // State 9
        -64,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -37,
        // State 13
        -45,
        // State 14
        -36,
        // State 15
        0,
        // State 16
        -38,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        -42,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -30,
        // State 33
        -17,
        // State 34
        -18,
        // State 35
        -29,
        // State 36
        -32,
        // State 37
        -33,
        // State 38
        -40,
        // State 39
        -41,
        // State 40
        -52,
        // State 41
        -53,
        // State 42
        -54,
        // State 43
        -55,
        // State 44
        -46,
        // State 45
        0,
        // State 46
        -57,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 4, 0, 0, 5, 6, 7, 8, 0, 0, 9, 10, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 31, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 34, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 35, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 37, 0, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 38, 0, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 39, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 40, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 47, 0, 0, 5, 6, 7, 8, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!""###,
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""?""###,
            r###""[""###,
            r###""]""###,
            r###""and""###,
            r###""else""###,
            r###""false""###,
            r###""float""###,
            r###""for""###,
            r###""fun""###,
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""let""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 36)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct StatementParser {
        _priv: (),
    }

    impl StatementParser {
        pub fn new() -> StatementParser {
            StatementParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<Error=lexer::LexicalError>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<ast::Statement, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    lexer::Token::ExclMark if true => 0,
                    lexer::Token::NotEqual if true => 1,
                    lexer::Token::ParOpen if true => 2,
                    lexer::Token::ParClose if true => 3,
                    lexer::Token::Star if true => 4,
                    lexer::Token::Plus if true => 5,
                    lexer::Token::Comma if true => 6,
                    lexer::Token::Minus if true => 7,
                    lexer::Token::Dot if true => 8,
                    lexer::Token::Slash if true => 9,
                    lexer::Token::Colon if true => 10,
                    lexer::Token::Semicolon if true => 11,
                    lexer::Token::Less if true => 12,
                    lexer::Token::LessEqual if true => 13,
                    lexer::Token::Equal if true => 14,
                    lexer::Token::VeryEqual if true => 15,
                    lexer::Token::Greater if true => 16,
                    lexer::Token::GreaterEqual if true => 17,
                    lexer::Token::Question if true => 18,
                    lexer::Token::BracketOpen if true => 19,
                    lexer::Token::BracketClose if true => 20,
                    lexer::Token::And if true => 21,
                    lexer::Token::Else if true => 22,
                    lexer::Token::False if true => 23,
                    lexer::Token::FloatLiteral(_) if true => 24,
                    lexer::Token::For if true => 25,
                    lexer::Token::Fun if true => 26,
                    lexer::Token::Identifier(_) if true => 27,
                    lexer::Token::If if true => 28,
                    lexer::Token::IntLiteral(_) if true => 29,
                    lexer::Token::Let if true => 30,
                    lexer::Token::Or if true => 31,
                    lexer::Token::StringLiteral(_) if true => 32,
                    lexer::Token::True if true => 33,
                    lexer::Token::BraceOpen if true => 34,
                    lexer::Token::BraceClose if true => 35,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 36 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                __tok @ lexer::Token::ExclMark => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                __tok @ lexer::Token::NotEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                __tok @ lexer::Token::ParOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                __tok @ lexer::Token::ParClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                __tok @ lexer::Token::Star => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                __tok @ lexer::Token::Plus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                __tok @ lexer::Token::Comma => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                __tok @ lexer::Token::Minus => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                __tok @ lexer::Token::Dot => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                __tok @ lexer::Token::Slash => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                __tok @ lexer::Token::Colon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                __tok @ lexer::Token::Semicolon => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                __tok @ lexer::Token::Less => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            13 => match __lookahead.1 {
                                __tok @ lexer::Token::LessEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            14 => match __lookahead.1 {
                                __tok @ lexer::Token::Equal => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            15 => match __lookahead.1 {
                                __tok @ lexer::Token::VeryEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            16 => match __lookahead.1 {
                                __tok @ lexer::Token::Greater => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            17 => match __lookahead.1 {
                                __tok @ lexer::Token::GreaterEqual => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            18 => match __lookahead.1 {
                                __tok @ lexer::Token::Question => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            19 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            20 => match __lookahead.1 {
                                __tok @ lexer::Token::BracketClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            21 => match __lookahead.1 {
                                __tok @ lexer::Token::And => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            22 => match __lookahead.1 {
                                __tok @ lexer::Token::Else => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            23 => match __lookahead.1 {
                                __tok @ lexer::Token::False => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            24 => match __lookahead.1 {
                                lexer::Token::FloatLiteral(__tok0) => __Symbol::Variant1((__tok0)),
                                _ => unreachable!(),
                            },
                            25 => match __lookahead.1 {
                                __tok @ lexer::Token::For => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            26 => match __lookahead.1 {
                                __tok @ lexer::Token::Fun => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                __tok @ lexer::Token::Let => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            34 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            35 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceClose => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Statement,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            2 => {
                __reduce2(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            3 => {
                __reduce3(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            4 => {
                __reduce4(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            5 => {
                __reduce5(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            6 => {
                __reduce6(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            7 => {
                __reduce7(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            8 => {
                __reduce8(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            9 => {
                __reduce9(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            10 => {
                __reduce10(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            11 => {
                __reduce11(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            12 => {
                __reduce12(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            13 => {
                __reduce13(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            14 => {
                __reduce14(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            15 => {
                __reduce15(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            16 => {
                __reduce16(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            17 => {
                __reduce17(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            18 => {
                __reduce18(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            19 => {
                __reduce19(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            20 => {
                __reduce20(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            21 => {
                __reduce21(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            22 => {
                __reduce22(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            23 => {
                __reduce23(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            24 => {
                __reduce24(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            25 => {
                __reduce25(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            26 => {
                __reduce26(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            27 => {
                __reduce27(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            28 => {
                __reduce28(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            29 => {
                __reduce29(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            30 => {
                __reduce30(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            33 => {
                __reduce33(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            34 => {
                __reduce34(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            35 => {
                __reduce35(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            36 => {
                __reduce36(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            37 => {
                __reduce37(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            38 => {
                __reduce38(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            39 => {
                __reduce39(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            40 => {
                __reduce40(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            41 => {
                __reduce41(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            42 => {
                __reduce42(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            43 => {
                __reduce43(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            44 => {
                __reduce44(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            45 => {
                __reduce45(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            46 => {
                __reduce46(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            47 => {
                __reduce47(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            48 => {
                __reduce48(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            49 => {
                __reduce49(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            50 => {
                __reduce50(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            51 => {
                __reduce51(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            52 => {
                __reduce52(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            53 => {
                __reduce53(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            54 => {
                __reduce54(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            55 => {
                __reduce55(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            56 => {
                __reduce56(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            57 => {
                __reduce57(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            58 => {
                __reduce58(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            59 => {
                __reduce59(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            60 => {
                __reduce60(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            61 => {
                __reduce61(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            62 => {
                __reduce62(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            63 => {
                __reduce63(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            64 => {
                // __Statement = Statement => ActionFn(4);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 33 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, (ast::Variable, String), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant12<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant12(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant11<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Block, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant11(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant10<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant10(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant14<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Function, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant14(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant15<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::GpuFunction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant15(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant16<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant16(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant17<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant17(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant8<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::ProgramItem, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant8(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, i64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, lexer::Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant13<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::option::Option<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant13(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<(ast::Variable, String)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant9<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ProgramItem>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant9(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",") = Definition, "," => ActionFn(54);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action54::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* =  => ActionFn(52);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action52::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")* = (<Definition> ",")+ => ActionFn(53);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action53::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = Definition, "," => ActionFn(59);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action59::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Definition> ",")+ = (<Definition> ",")+, Definition, "," => ActionFn(60);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action60::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";") = Statement, ";" => ActionFn(43);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action43::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (2, __symbol, 3)
    }
    pub(crate) fn __reduce7<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* =  => ActionFn(41);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action41::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce8<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")* = (<Statement> ";")+ => ActionFn(42);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action42::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 4)
    }
    pub(crate) fn __reduce9<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = Statement, ";" => ActionFn(63);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action63::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (2, __symbol, 5)
    }
    pub(crate) fn __reduce10<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (<Statement> ";")+ = (<Statement> ";")+, Statement, ";" => ActionFn(64);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant6(__symbols);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action64::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (3, __symbol, 5)
    }
    pub(crate) fn __reduce11<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem) = ProgramItem => ActionFn(47);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* =  => ActionFn(45);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action45::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (0, __symbol, 7)
    }
    pub(crate) fn __reduce13<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)* = (ProgramItem)+ => ActionFn(46);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = ProgramItem => ActionFn(67);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // (ProgramItem)+ = (ProgramItem)+, ProgramItem => ActionFn(68);
        let __sym1 = __pop_Variant8(__symbols);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action68::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant9(__nt), __end);
        (2, __symbol, 8)
    }
    pub(crate) fn __reduce16<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Multiplied => ActionFn(27);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "+", Multiplied => ActionFn(28);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce18<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Added = Added, "-", Multiplied => ActionFn(29);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action29::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 9)
    }
    pub(crate) fn __reduce19<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", "}" => ActionFn(65);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action65::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce20<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Block = "{", (<Statement> ";")+, "}" => ActionFn(66);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant7(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action66::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (3, __symbol, 10)
    }
    pub(crate) fn __reduce21<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = Definition => ActionFn(71);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce22<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> =  => ActionFn(72);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action72::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (0, __symbol, 11)
    }
    pub(crate) fn __reduce23<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+, Definition => ActionFn(73);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action73::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (2, __symbol, 11)
    }
    pub(crate) fn __reduce24<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Comma<Definition> = (<Definition> ",")+ => ActionFn(74);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant12(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce25<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition = "ident", ":", "ident" => ActionFn(12);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 12)
    }
    pub(crate) fn __reduce26<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? = Definition => ActionFn(50);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action50::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce27<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Definition? =  => ActionFn(51);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action51::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant13(__nt), __end);
        (0, __symbol, 13)
    }
    pub(crate) fn __reduce28<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Related => ActionFn(19);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action19::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce29<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "==", Added => ActionFn(20);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce30<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Equaled = Added, "!=", Added => ActionFn(21);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 14)
    }
    pub(crate) fn __reduce31<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Equaled => ActionFn(16);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce32<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "and", Equaled => ActionFn(17);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce33<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Expression = Expression, "or", Equaled => ActionFn(18);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 15)
    }
    pub(crate) fn __reduce34<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Function = "fun", "ident", "(", Comma<Definition>, ")", Block => ActionFn(10);
        let __sym5 = __pop_Variant11(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (6, __symbol, 16)
    }
    pub(crate) fn __reduce35<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // GpuFunction = "fun", "ident", "(", Comma<Definition>, ")", "{", "string", "}" => ActionFn(11);
        let __sym7 = __pop_Variant0(__symbols);
        let __sym6 = __pop_Variant2(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant12(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym7.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (8, __symbol, 17)
    }
    pub(crate) fn __reduce36<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "int" => ActionFn(38);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce37<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "float" => ActionFn(39);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce38<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Literal = "string" => ActionFn(40);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant16(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce39<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Negated => ActionFn(30);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce40<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "*", Negated => ActionFn(31);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce41<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Multiplied = Multiplied, "/", Negated => ActionFn(32);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 19)
    }
    pub(crate) fn __reduce42<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = "!", Primitive => ActionFn(33);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (2, __symbol, 20)
    }
    pub(crate) fn __reduce43<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Negated = Primitive => ActionFn(34);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action34::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce44<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = Literal => ActionFn(35);
        let __sym0 = __pop_Variant16(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce45<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "ident" => ActionFn(36);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce46<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Primitive = "(", Expression, ")" => ActionFn(37);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant10(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action37::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 21)
    }
    pub(crate) fn __reduce47<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program =  => ActionFn(69);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action69::<>(&__start, &__end);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (0, __symbol, 22)
    }
    pub(crate) fn __reduce48<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Program = (ProgramItem)+ => ActionFn(70);
        let __sym0 = __pop_Variant9(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 22)
    }
    pub(crate) fn __reduce49<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = Function => ActionFn(8);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce50<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // ProgramItem = GpuFunction => ActionFn(9);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant8(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce51<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Added => ActionFn(22);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce52<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<", Added => ActionFn(23);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce53<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, "<=", Added => ActionFn(24);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce54<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">", Added => ActionFn(25);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce55<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Related = Related, ">=", Added => ActionFn(26);
        let __sym2 = __pop_Variant10(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action26::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (3, __symbol, 24)
    }
    pub(crate) fn __reduce56<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = Expression => ActionFn(14);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce57<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // Statement = "let", "ident", "=", Expression => ActionFn(15);
        let __sym3 = __pop_Variant10(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action15::<>(__sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (4, __symbol, 25)
    }
    pub(crate) fn __reduce58<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Block = Block => ActionFn(3);
        let __sym0 = __pop_Variant11(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant11(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce59<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Expression = Expression => ActionFn(5);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce60<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Function = Function => ActionFn(1);
        let __sym0 = __pop_Variant14(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant14(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce61<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __GpuFunction = GpuFunction => ActionFn(2);
        let __sym0 = __pop_Variant15(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant15(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce62<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Primitive = Primitive => ActionFn(6);
        let __sym0 = __pop_Variant10(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant10(__nt), __end);
        (1, __symbol, 30)
    }
    pub(crate) fn __reduce63<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, (usize,__Symbol<>,usize), usize)
    {
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant17(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant17(__nt), __end);
        (1, __symbol, 31)
    }
}
pub use self::__parse__Statement::StatementParser;

fn __action0<
>(
    (_, __0, _): (usize, ast::Program, usize),
) -> ast::Program
{
    (__0)
}

fn __action1<
>(
    (_, __0, _): (usize, ast::Function, usize),
) -> ast::Function
{
    (__0)
}

fn __action2<
>(
    (_, __0, _): (usize, ast::GpuFunction, usize),
) -> ast::GpuFunction
{
    (__0)
}

fn __action3<
>(
    (_, __0, _): (usize, ast::Block, usize),
) -> ast::Block
{
    (__0)
}

fn __action4<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ast::Statement
{
    (__0)
}

fn __action5<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    (__0)
}

fn __action6<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    (__0)
}

fn __action7<
>(
    (_, items, _): (usize, ::std::vec::Vec<ast::ProgramItem>, usize),
) -> ast::Program
{
    ast::Program{items}
}

fn __action8<
>(
    (_, __0, _): (usize, ast::Function, usize),
) -> ast::ProgramItem
{
    ast::ProgramItem::Function(Box::new(__0))
}

fn __action9<
>(
    (_, __0, _): (usize, ast::GpuFunction, usize),
) -> ast::ProgramItem
{
    ast::ProgramItem::GpuFunction(Box::new(__0))
}

fn __action10<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, i, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, args, _): (usize, Vec<(ast::Variable, String)>, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, b, _): (usize, ast::Block, usize),
) -> ast::Function
{
    ast::Function{arguments: args, name: i, block: b}
}

fn __action11<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, i, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, args, _): (usize, Vec<(ast::Variable, String)>, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, s, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::GpuFunction
{
    ast::GpuFunction{ name:i, code: s, arguments: args}
}

fn __action12<
>(
    (_, i1, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, i2, _): (usize, String, usize),
) -> (ast::Variable, String)
{
    (ast::Variable{identifier: i1}, i2)
}

fn __action13<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, statements, _): (usize, ::std::vec::Vec<ast::Statement>, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Block
{
    ast::Block{statements: statements }
}

fn __action14<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Statement
{
    ast::Statement::Expression(__0)
}

fn __action15<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, i, _): (usize, String, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, e, _): (usize, ast::Expression, usize),
) -> ast::Statement
{
    ast::Statement::Assignment(Box::new(ast::Variable{identifier: i}), e)
}

fn __action16<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action17<
>(
    (_, e1, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, e2, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::And(Box::new(e1), Box::new(e2))
}

fn __action18<
>(
    (_, e1, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, e2, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Or(Box::new(e1), Box::new(e2))
}

fn __action19<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action20<
>(
    (_, a1, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a2, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Equals(Box::new(a1), Box::new(a2))
}

fn __action21<
>(
    (_, a1, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a2, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::NotEquals(Box::new(a1), Box::new(a2))
}

fn __action22<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action23<
>(
    (_, r, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Less(Box::new(r), Box::new(a))
}

fn __action24<
>(
    (_, r, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::LessEqual(Box::new(r), Box::new(a))
}

fn __action25<
>(
    (_, r, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::More(Box::new(r), Box::new(a))
}

fn __action26<
>(
    (_, r, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::MoreEqual(Box::new(r), Box::new(a))
}

fn __action27<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action28<
>(
    (_, a, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, m, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Add(Box::new(a), Box::new(m))
}

fn __action29<
>(
    (_, a, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, m, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Sub(Box::new(a), Box::new(m))
}

fn __action30<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action31<
>(
    (_, m, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, n, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Mul(Box::new(m), Box::new(n))
}

fn __action32<
>(
    (_, m, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, n, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Div(Box::new(m), Box::new(n))
}

fn __action33<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, p, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    ast::Expression::Negation(Box::new(p))
}

fn __action34<
>(
    (_, __0, _): (usize, ast::Expression, usize),
) -> ast::Expression
{
    __0
}

fn __action35<
>(
    (_, __0, _): (usize, ast::Literal, usize),
) -> ast::Expression
{
    ast::Expression::Literal(Box::new(__0))
}

fn __action36<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Expression
{
    ast::Expression::Variable(Box::new(ast::Variable {identifier: __0}))
}

fn __action37<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, e, _): (usize, ast::Expression, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Expression
{
    e
}

fn __action38<
>(
    (_, __0, _): (usize, i64, usize),
) -> ast::Literal
{
    ast::Literal::Int(__0)
}

fn __action39<
>(
    (_, __0, _): (usize, f64, usize),
) -> ast::Literal
{
    ast::Literal::Float(__0)
}

fn __action40<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Literal
{
    ast::Literal::String(__0)
}

fn __action41<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::Statement>
{
    vec![]
}

fn __action42<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::Statement>, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    v
}

fn __action43<
>(
    (_, __0, _): (usize, ast::Statement, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Statement
{
    (__0)
}

fn __action44<
>(
    (_, v, _): (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
    (_, e, _): (usize, ::std::option::Option<(ast::Variable, String)>, usize),
) -> Vec<(ast::Variable, String)>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

fn __action45<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::ProgramItem>
{
    vec![]
}

fn __action46<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::ProgramItem>, usize),
) -> ::std::vec::Vec<ast::ProgramItem>
{
    v
}

fn __action47<
>(
    (_, __0, _): (usize, ast::ProgramItem, usize),
) -> ast::ProgramItem
{
    (__0)
}

fn __action48<
>(
    (_, __0, _): (usize, ast::ProgramItem, usize),
) -> ::std::vec::Vec<ast::ProgramItem>
{
    vec![__0]
}

fn __action49<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::ProgramItem>, usize),
    (_, e, _): (usize, ast::ProgramItem, usize),
) -> ::std::vec::Vec<ast::ProgramItem>
{
    { let mut v = v; v.push(e); v }
}

fn __action50<
>(
    (_, __0, _): (usize, (ast::Variable, String), usize),
) -> ::std::option::Option<(ast::Variable, String)>
{
    Some(__0)
}

fn __action51<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<(ast::Variable, String)>
{
    None
}

fn __action52<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    vec![]
}

fn __action53<
>(
    (_, v, _): (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    v
}

fn __action54<
>(
    (_, __0, _): (usize, (ast::Variable, String), usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> (ast::Variable, String)
{
    (__0)
}

fn __action55<
>(
    (_, __0, _): (usize, ast::Statement, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    vec![__0]
}

fn __action56<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::Statement>, usize),
    (_, e, _): (usize, ast::Statement, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    { let mut v = v; v.push(e); v }
}

fn __action57<
>(
    (_, __0, _): (usize, (ast::Variable, String), usize),
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    vec![__0]
}

fn __action58<
>(
    (_, v, _): (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
    (_, e, _): (usize, (ast::Variable, String), usize),
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    { let mut v = v; v.push(e); v }
}

fn __action59<
>(
    __0: (usize, (ast::Variable, String), usize),
    __1: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action54(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        __temp0,
    )
}

fn __action60<
>(
    __0: (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
    __1: (usize, (ast::Variable, String), usize),
    __2: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<(ast::Variable, String)>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action54(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __0,
        __temp0,
    )
}

fn __action61<
>(
    __0: (usize, ::std::option::Option<(ast::Variable, String)>, usize),
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        __temp0,
        __0,
    )
}

fn __action62<
>(
    __0: (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
    __1: (usize, ::std::option::Option<(ast::Variable, String)>, usize),
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        __temp0,
        __1,
    )
}

fn __action63<
>(
    __0: (usize, ast::Statement, usize),
    __1: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        __temp0,
    )
}

fn __action64<
>(
    __0: (usize, ::std::vec::Vec<ast::Statement>, usize),
    __1: (usize, ast::Statement, usize),
    __2: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<ast::Statement>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action43(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __temp0,
    )
}

fn __action65<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action41(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
        __1,
    )
}

fn __action66<
>(
    __0: (usize, lexer::Token, usize),
    __1: (usize, ::std::vec::Vec<ast::Statement>, usize),
    __2: (usize, lexer::Token, usize),
) -> ast::Block
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action42(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action13(
        __0,
        __temp0,
        __2,
    )
}

fn __action67<
>(
    __0: (usize, ast::ProgramItem, usize),
) -> ::std::vec::Vec<ast::ProgramItem>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action47(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        __temp0,
    )
}

fn __action68<
>(
    __0: (usize, ::std::vec::Vec<ast::ProgramItem>, usize),
    __1: (usize, ast::ProgramItem, usize),
) -> ::std::vec::Vec<ast::ProgramItem>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        __0,
        __temp0,
    )
}

fn __action69<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ast::Program
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action45(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __temp0,
    )
}

fn __action70<
>(
    __0: (usize, ::std::vec::Vec<ast::ProgramItem>, usize),
) -> ast::Program
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action46(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        __temp0,
    )
}

fn __action71<
>(
    __0: (usize, (ast::Variable, String), usize),
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action50(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __temp0,
    )
}

fn __action72<
>(
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action51(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        __temp0,
    )
}

fn __action73<
>(
    __0: (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
    __1: (usize, (ast::Variable, String), usize),
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __0,
        __temp0,
    )
}

fn __action74<
>(
    __0: (usize, ::std::vec::Vec<(ast::Variable, String)>, usize),
) -> Vec<(ast::Variable, String)>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __0,
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize),Self::Error>;
}

impl<> __ToTriple<> for (usize, lexer::Token, usize) {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize),lexer::LexicalError> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, lexer::Token, usize),lexer::LexicalError> {
    type Error = lexer::LexicalError;
    fn to_triple(value: Self) -> Result<(usize,lexer::Token,usize),lexer::LexicalError> {
        value
    }
}
