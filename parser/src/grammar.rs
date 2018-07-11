// auto-generated: "lalrpop 0.15.2"
// sha256: 3897f9d061d2b18b7f6505cba03ae833f9d55bf1e15f9287d5a7e7b05bca1
use std::str::FromStr;
use lexer;
use ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

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
        Variant4(ast::Primitive),
        Variant5(::std::vec::Vec<ast::Primitive>),
        Variant6(ast::Literal),
        Variant7(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 1
        0, 16, 0, -25, 0, 17, 0, 18, 0, 0, 0, 0, -25, -25, 0, 19, -25, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0,
        // State 2
        0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 4
        0, -21, 0, -21, -21, -21, 0, -21, 0, -21, 0, 0, -21, -21, 0, -21, -21, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 5
        0, -4, 0, -4, 22, -4, 0, -4, 0, 23, 0, 0, -4, -4, 0, -4, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 6
        0, -16, 0, -16, -16, -16, 0, -16, 0, -16, 0, 0, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 7
        0, -20, 0, -20, -20, -20, 0, -20, 0, -20, 0, 0, -20, -20, 0, -20, -20, -20, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 8
        0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 24, 25, 0, 0, 26, 27, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0,
        // State 9
        0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 10
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 11
        0, -14, 0, -14, -14, -14, 0, -14, 0, -14, 0, 0, -14, -14, 0, -14, -14, -14, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0,
        // State 12
        0, -22, 0, -22, -22, -22, 0, -22, 0, -22, 0, 0, -22, -22, 0, -22, -22, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 13
        0, -13, 0, -13, -13, -13, 0, -13, 0, -13, 0, 0, -13, -13, 0, -13, -13, -13, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0,
        // State 14
        0, -15, 0, -15, -15, -15, 0, -15, 0, -15, 0, 0, -15, -15, 0, -15, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 15
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 16
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 17
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 18
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 19
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 20
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 21
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 22
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 23
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 24
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 25
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 26
        10, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 13, 0, 14, 0, 15, 0, 0, 0,
        // State 27
        0, -19, 0, -19, -19, -19, 0, -19, 0, -19, 0, 0, -19, -19, 0, -19, -19, -19, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0,
        // State 29
        0, 0, 0, -9, 0, 17, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0,
        // State 30
        0, -5, 0, -5, 22, -5, 0, -5, 0, 23, 0, 0, -5, -5, 0, -5, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0,
        // State 31
        0, -6, 0, -6, 22, -6, 0, -6, 0, 23, 0, 0, -6, -6, 0, -6, -6, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -8, 0, 17, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0,
        // State 34
        0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0,
        // State 35
        0, -17, 0, -17, -17, -17, 0, -17, 0, -17, 0, 0, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 36
        0, -18, 0, -18, -18, -18, 0, -18, 0, -18, 0, 0, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 37
        0, 0, 0, -26, 0, 17, 0, 18, 0, 0, 0, 0, -26, -26, 0, 0, -26, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0,
        // State 38
        0, 0, 0, -27, 0, 17, 0, 18, 0, 0, 0, 0, -27, -27, 0, 0, -27, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0,
        // State 39
        0, 0, 0, -28, 0, 17, 0, 18, 0, 0, 0, 0, -28, -28, 0, 0, -28, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -29, 0, 17, 0, 18, 0, 0, 0, 0, -29, -29, 0, 0, -29, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 41
        0, -23, 0, -23, -23, -23, 0, -23, 0, -23, 0, 0, -23, -23, 0, -23, -23, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -25,
        // State 2
        -10,
        // State 3
        -30,
        // State 4
        -21,
        // State 5
        -4,
        // State 6
        -16,
        // State 7
        -20,
        // State 8
        -7,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -14,
        // State 12
        -22,
        // State 13
        -13,
        // State 14
        -15,
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
        -19,
        // State 28
        0,
        // State 29
        -9,
        // State 30
        -5,
        // State 31
        -6,
        // State 32
        -8,
        // State 33
        -11,
        // State 34
        -12,
        // State 35
        -17,
        // State 36
        -18,
        // State 37
        -26,
        // State 38
        -27,
        // State 39
        -28,
        // State 40
        -29,
        // State 41
        -23,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 2, 3, 4, 5, 6, 7, 8, 0, 9, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 5, 0, 0, 28, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 2, 3, 29, 5, 6, 7, 8, 0, 9, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 30, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 5, 31, 7, 8, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 5, 32, 7, 8, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 33, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 2, 34, 0, 5, 6, 7, 8, 0, 9, 0, 0, 0,
        // State 20
        0, 0, 2, 35, 0, 5, 6, 7, 8, 0, 9, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 5, 0, 36, 8, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 5, 0, 37, 8, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 38, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 39, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 40, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 41, 0, 0, 5, 6, 7, 8, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 34)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
        ) -> Result<ast::Primitive, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
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
                    lexer::Token::Identifier(_) if true => 26,
                    lexer::Token::If if true => 27,
                    lexer::Token::IntLiteral(_) if true => 28,
                    lexer::Token::Or if true => 29,
                    lexer::Token::StringLiteral(_) if true => 30,
                    lexer::Token::True if true => 31,
                    lexer::Token::BraceOpen if true => 32,
                    lexer::Token::BraceClose if true => 33,
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
                    let __action = __ACTION[__state * 34 + __integer];
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
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
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
    ) -> Option<Result<ast::Primitive,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
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
                // __Expression = Expression => ActionFn(1);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(__sym0);
                return Some(Ok(__nt));
            }
            31 => {
                __reduce31(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Primitive, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Primitive>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
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
        // (<Expression> ";") = Expression, ";" => ActionFn(31);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
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
        // (<Expression> ";")+ = Expression, ";" => ActionFn(32);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 1)
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
        // (<Expression> ";")+ = (<Expression> ";")+, Expression, ";" => ActionFn(33);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 1)
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
        // Added = Multiplied => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 2)
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
        // Added = Added, "+", Multiplied => ActionFn(16);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
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
        // Added = Added, "-", Multiplied => ActionFn(17);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 2)
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
        // Equaled = Related => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 3)
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
        // Equaled = Added, "==", Added => ActionFn(8);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Equaled = Added, "!=", Added => ActionFn(9);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Expression = Equaled => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
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
        // Expression = Expression, "and", Equaled => ActionFn(5);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Expression = Expression, "or", Equaled => ActionFn(6);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Literal = "int" => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "float" => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "string" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Multiplied = Negated => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 6)
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
        // Multiplied = Multiplied, "*", Negated => ActionFn(19);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Multiplied = Multiplied, "/", Negated => ActionFn(20);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Negated = "!", Primitive => ActionFn(21);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 7)
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
        // Negated = Primitive => ActionFn(22);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
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
        // Primitive = Literal => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "ident" => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "(", Expression, ")" => ActionFn(25);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 8)
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
        // Program = (<Expression> ";")+ => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
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
        // Related = Added => ActionFn(10);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 10)
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
        // Related = Related, "<", Added => ActionFn(11);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, "<=", Added => ActionFn(12);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">", Added => ActionFn(13);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">=", Added => ActionFn(14);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // __Primitive = Primitive => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 12)
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
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 13)
    }
}
pub use self::__parse__Expression::ExpressionParser;

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
        Variant4(ast::Primitive),
        Variant5(::std::vec::Vec<ast::Primitive>),
        Variant6(ast::Literal),
        Variant7(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 1
        0, -21, 0, -21, -21, -21, 0, -21, 0, -21, 0, 0, -21, -21, 0, -21, -21, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 4
        0, -14, 0, -14, -14, -14, 0, -14, 0, -14, 0, 0, -14, -14, 0, -14, -14, -14, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0,
        // State 5
        0, -22, 0, -22, -22, -22, 0, -22, 0, -22, 0, 0, -22, -22, 0, -22, -22, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 6
        0, -13, 0, -13, -13, -13, 0, -13, 0, -13, 0, 0, -13, -13, 0, -13, -13, -13, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0,
        // State 7
        0, -15, 0, -15, -15, -15, 0, -15, 0, -15, 0, 0, -15, -15, 0, -15, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 8
        0, 17, 0, -25, 0, 18, 0, 19, 0, 0, 0, 0, -25, -25, 0, 20, -25, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0,
        // State 9
        0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0,
        // State 11
        0, -4, 0, -4, 24, -4, 0, -4, 0, 25, 0, 0, -4, -4, 0, -4, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 12
        0, -16, 0, -16, -16, -16, 0, -16, 0, -16, 0, 0, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 13
        0, -20, 0, -20, -20, -20, 0, -20, 0, -20, 0, 0, -20, -20, 0, -20, -20, -20, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 14
        0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 26, 27, 0, 0, 28, 29, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0,
        // State 15
        0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 16
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 17
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 18
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 19
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 20
        0, -23, 0, -23, -23, -23, 0, -23, 0, -23, 0, 0, -23, -23, 0, -23, -23, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0,
        // State 21
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 22
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 23
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 24
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 25
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 26
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 27
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 28
        16, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 6, 0, 7, 0, 8, 0, 0, 0,
        // State 29
        0, -19, 0, -19, -19, -19, 0, -19, 0, -19, 0, 0, -19, -19, 0, -19, -19, -19, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0,
        // State 30
        0, 0, 0, -9, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0,
        // State 31
        0, -5, 0, -5, 24, -5, 0, -5, 0, 25, 0, 0, -5, -5, 0, -5, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0,
        // State 32
        0, -6, 0, -6, 24, -6, 0, -6, 0, 25, 0, 0, -6, -6, 0, -6, -6, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0,
        // State 33
        0, 0, 0, -8, 0, 18, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0,
        // State 34
        0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0,
        // State 35
        0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0,
        // State 36
        0, -17, 0, -17, -17, -17, 0, -17, 0, -17, 0, 0, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 37
        0, -18, 0, -18, -18, -18, 0, -18, 0, -18, 0, 0, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 38
        0, 0, 0, -26, 0, 18, 0, 19, 0, 0, 0, 0, -26, -26, 0, 0, -26, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0,
        // State 39
        0, 0, 0, -27, 0, 18, 0, 19, 0, 0, 0, 0, -27, -27, 0, 0, -27, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0,
        // State 40
        0, 0, 0, -28, 0, 18, 0, 19, 0, 0, 0, 0, -28, -28, 0, 0, -28, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 41
        0, 0, 0, -29, 0, 18, 0, 19, 0, 0, 0, 0, -29, -29, 0, 0, -29, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -21,
        // State 2
        -31,
        // State 3
        0,
        // State 4
        -14,
        // State 5
        -22,
        // State 6
        -13,
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
        0,
        // State 20
        -23,
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
        0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 9, 10, 11, 2, 12, 13, 14, 0, 15, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 2, 0, 0, 30, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 31, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 2, 32, 13, 14, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 2, 33, 13, 14, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 34, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 9, 35, 0, 2, 12, 13, 14, 0, 15, 0, 0, 0,
        // State 22
        0, 0, 9, 36, 0, 2, 12, 13, 14, 0, 15, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 2, 0, 37, 14, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 2, 0, 38, 14, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 39, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 40, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 41, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 42, 0, 0, 2, 12, 13, 14, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 34)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
        ) -> Result<ast::Primitive, __lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>
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
                    lexer::Token::Identifier(_) if true => 26,
                    lexer::Token::If if true => 27,
                    lexer::Token::IntLiteral(_) if true => 28,
                    lexer::Token::Or if true => 29,
                    lexer::Token::StringLiteral(_) if true => 30,
                    lexer::Token::True if true => 31,
                    lexer::Token::BraceOpen if true => 32,
                    lexer::Token::BraceClose if true => 33,
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
                    let __action = __ACTION[__state * 34 + __integer];
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
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
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
    ) -> Option<Result<ast::Primitive,__lalrpop_util::ParseError<usize, lexer::Token, lexer::LexicalError>>>
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
                // __Primitive = Primitive => ActionFn(2);
                let __sym0 = __pop_Variant4(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(__sym0);
                return Some(Ok(__nt));
            }
            32 => {
                __reduce32(__action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<()>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Primitive, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Primitive>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
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
        // (<Expression> ";") = Expression, ";" => ActionFn(31);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
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
        // (<Expression> ";")+ = Expression, ";" => ActionFn(32);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 1)
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
        // (<Expression> ";")+ = (<Expression> ";")+, Expression, ";" => ActionFn(33);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 1)
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
        // Added = Multiplied => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 2)
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
        // Added = Added, "+", Multiplied => ActionFn(16);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
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
        // Added = Added, "-", Multiplied => ActionFn(17);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 2)
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
        // Equaled = Related => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 3)
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
        // Equaled = Added, "==", Added => ActionFn(8);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Equaled = Added, "!=", Added => ActionFn(9);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Expression = Equaled => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
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
        // Expression = Expression, "and", Equaled => ActionFn(5);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Expression = Expression, "or", Equaled => ActionFn(6);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Literal = "int" => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "float" => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "string" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Multiplied = Negated => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 6)
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
        // Multiplied = Multiplied, "*", Negated => ActionFn(19);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Multiplied = Multiplied, "/", Negated => ActionFn(20);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Negated = "!", Primitive => ActionFn(21);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 7)
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
        // Negated = Primitive => ActionFn(22);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
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
        // Primitive = Literal => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "ident" => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "(", Expression, ")" => ActionFn(25);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 8)
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
        // Program = (<Expression> ";")+ => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
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
        // Related = Added => ActionFn(10);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 10)
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
        // Related = Related, "<", Added => ActionFn(11);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, "<=", Added => ActionFn(12);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">", Added => ActionFn(13);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">=", Added => ActionFn(14);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // __Expression = Expression => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 11)
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
        // __Program = Program => ActionFn(0);
        let __sym0 = __pop_Variant7(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action0::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 13)
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
        Variant4(ast::Primitive),
        Variant5(::std::vec::Vec<ast::Primitive>),
        Variant6(ast::Literal),
        Variant7(ast::Program),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 1
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 2
        0, 19, 0, -25, 0, 20, 0, 21, 0, 0, 0, -25, -25, -25, 0, 22, -25, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0,
        // State 3
        0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0,
        // State 5
        0, -21, 0, -21, -21, -21, 0, -21, 0, -21, 0, -21, -21, -21, 0, -21, -21, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, 0,
        // State 6
        0, -4, 0, -4, 26, -4, 0, -4, 0, 27, 0, -4, -4, -4, 0, -4, -4, -4, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0,
        // State 7
        0, -16, 0, -16, -16, -16, 0, -16, 0, -16, 0, -16, -16, -16, 0, -16, -16, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0,
        // State 8
        0, -20, 0, -20, -20, -20, 0, -20, 0, -20, 0, -20, -20, -20, 0, -20, -20, -20, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, -7, 28, 29, 0, 0, 30, 31, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0,
        // State 11
        0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 12
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 13
        0, -14, 0, -14, -14, -14, 0, -14, 0, -14, 0, -14, -14, -14, 0, -14, -14, -14, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0,
        // State 14
        0, -22, 0, -22, -22, -22, 0, -22, 0, -22, 0, -22, -22, -22, 0, -22, -22, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, 0,
        // State 15
        0, -13, 0, -13, -13, -13, 0, -13, 0, -13, 0, -13, -13, -13, 0, -13, -13, -13, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0,
        // State 16
        0, -15, 0, -15, -15, -15, 0, -15, 0, -15, 0, -15, -15, -15, 0, -15, -15, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0,
        // State 18
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 19
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 20
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 21
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 22
        -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, -2, 0, -2, 0, -2, 0, 0, 0,
        // State 23
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 24
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 25
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 26
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 27
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 28
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 29
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 30
        12, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 16, 0, 17, 0, 0, 0,
        // State 31
        0, -19, 0, -19, -19, -19, 0, -19, 0, -19, 0, -19, -19, -19, 0, -19, -19, -19, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0,
        // State 33
        -3, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, -3, 0, -3, 0, -3, 0, 0, 0,
        // State 34
        0, 0, 0, -9, 0, 20, 0, 21, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, 0,
        // State 35
        0, -5, 0, -5, 26, -5, 0, -5, 0, 27, 0, -5, -5, -5, 0, -5, -5, -5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0,
        // State 36
        0, -6, 0, -6, 26, -6, 0, -6, 0, 27, 0, -6, -6, -6, 0, -6, -6, -6, 0, 0, 0, -6, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 0, 0,
        // State 37
        0, 0, 0, -8, 0, 20, 0, 21, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0,
        // State 38
        0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0,
        // State 39
        0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0,
        // State 40
        0, -17, 0, -17, -17, -17, 0, -17, 0, -17, 0, -17, -17, -17, 0, -17, -17, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0,
        // State 41
        0, -18, 0, -18, -18, -18, 0, -18, 0, -18, 0, -18, -18, -18, 0, -18, -18, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0, 0,
        // State 42
        0, 0, 0, -26, 0, 20, 0, 21, 0, 0, 0, -26, -26, -26, 0, 0, -26, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0,
        // State 43
        0, 0, 0, -27, 0, 20, 0, 21, 0, 0, 0, -27, -27, -27, 0, 0, -27, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0,
        // State 44
        0, 0, 0, -28, 0, 20, 0, 21, 0, 0, 0, -28, -28, -28, 0, 0, -28, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0,
        // State 45
        0, 0, 0, -29, 0, 20, 0, 21, 0, 0, 0, -29, -29, -29, 0, 0, -29, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0,
        // State 46
        0, -23, 0, -23, -23, -23, 0, -23, 0, -23, 0, -23, -23, -23, 0, -23, -23, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -24,
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
        -32,
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
        0,
        // State 21
        0,
        // State 22
        -2,
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
        -3,
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
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 0, 0,
        // State 1
        0, 0, 3, 4, 18, 6, 7, 8, 9, 0, 11, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 6, 0, 0, 32, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 3, 4, 33, 6, 7, 8, 9, 0, 11, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 35, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 6, 36, 8, 9, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 6, 37, 8, 9, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 38, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 3, 39, 0, 6, 7, 8, 9, 0, 11, 0, 0, 0,
        // State 24
        0, 0, 3, 40, 0, 6, 7, 8, 9, 0, 11, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 6, 0, 41, 9, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 6, 0, 42, 9, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 43, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 44, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 45, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 46, 0, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
            r###""ident""###,
            r###""if""###,
            r###""int""###,
            r###""or""###,
            r###""string""###,
            r###""true""###,
            r###""{""###,
            r###""}""###,
        ];
        __ACTION[(__state * 34)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                    lexer::Token::Identifier(_) if true => 26,
                    lexer::Token::If if true => 27,
                    lexer::Token::IntLiteral(_) if true => 28,
                    lexer::Token::Or if true => 29,
                    lexer::Token::StringLiteral(_) if true => 30,
                    lexer::Token::True if true => 31,
                    lexer::Token::BraceOpen if true => 32,
                    lexer::Token::BraceClose if true => 33,
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
                    let __action = __ACTION[__state * 34 + __integer];
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
                                lexer::Token::Identifier(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            27 => match __lookahead.1 {
                                __tok @ lexer::Token::If => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            28 => match __lookahead.1 {
                                lexer::Token::IntLiteral(__tok0) => __Symbol::Variant3((__tok0)),
                                _ => unreachable!(),
                            },
                            29 => match __lookahead.1 {
                                __tok @ lexer::Token::Or => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            30 => match __lookahead.1 {
                                lexer::Token::StringLiteral(__tok0) => __Symbol::Variant2((__tok0)),
                                _ => unreachable!(),
                            },
                            31 => match __lookahead.1 {
                                __tok @ lexer::Token::True => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            32 => match __lookahead.1 {
                                __tok @ lexer::Token::BraceOpen => __Symbol::Variant0((__tok)),
                                _ => unreachable!(),
                            },
                            33 => match __lookahead.1 {
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
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant7(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Variant6<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Literal, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Primitive, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant7<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ast::Program, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant7(__v), __r) => (__l, __v, __r),
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
    fn __pop_Variant5<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Primitive>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
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
        // (<Expression> ";") = Expression, ";" => ActionFn(31);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action31::<>(__sym0, __sym1);
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
        // (<Expression> ";")+ = Expression, ";" => ActionFn(32);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action32::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 1)
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
        // (<Expression> ";")+ = (<Expression> ";")+, Expression, ";" => ActionFn(33);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action33::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (3, __symbol, 1)
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
        // Added = Multiplied => ActionFn(15);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 2)
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
        // Added = Added, "+", Multiplied => ActionFn(16);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
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
        // Added = Added, "-", Multiplied => ActionFn(17);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 2)
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
        // Equaled = Related => ActionFn(7);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 3)
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
        // Equaled = Added, "==", Added => ActionFn(8);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Equaled = Added, "!=", Added => ActionFn(9);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 3)
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
        // Expression = Equaled => ActionFn(4);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 4)
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
        // Expression = Expression, "and", Equaled => ActionFn(5);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Expression = Expression, "or", Equaled => ActionFn(6);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 4)
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
        // Literal = "int" => ActionFn(26);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "float" => ActionFn(27);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Literal = "string" => ActionFn(28);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant6(__nt), __end);
        (1, __symbol, 5)
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
        // Multiplied = Negated => ActionFn(18);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action18::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 6)
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
        // Multiplied = Multiplied, "*", Negated => ActionFn(19);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Multiplied = Multiplied, "/", Negated => ActionFn(20);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action20::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 6)
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
        // Negated = "!", Primitive => ActionFn(21);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(__sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (2, __symbol, 7)
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
        // Negated = Primitive => ActionFn(22);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 7)
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
        // Primitive = Literal => ActionFn(23);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "ident" => ActionFn(24);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 8)
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
        // Primitive = "(", Expression, ")" => ActionFn(25);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action25::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 8)
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
        // Program = (<Expression> ";")+ => ActionFn(3);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant7(__nt), __end);
        (1, __symbol, 9)
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
        // Related = Added => ActionFn(10);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 10)
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
        // Related = Related, "<", Added => ActionFn(11);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, "<=", Added => ActionFn(12);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">", Added => ActionFn(13);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // Related = Related, ">=", Added => ActionFn(14);
        let __sym2 = __pop_Variant4(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(__sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (3, __symbol, 10)
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
        // __Expression = Expression => ActionFn(1);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 11)
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
        // __Primitive = Primitive => ActionFn(2);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 12)
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
>(
    (_, __0, _): (usize, ast::Program, usize),
) -> ast::Program
{
    (__0)
}

fn __action1<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    (__0)
}

fn __action2<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    (__0)
}

fn __action3<
>(
    (_, lines, _): (usize, ::std::vec::Vec<ast::Primitive>, usize),
) -> ast::Program
{
    ast::Program{lines: lines}
}

fn __action4<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action5<
>(
    (_, e1, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, e2, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::And(Box::new(e1), Box::new(e2))
}

fn __action6<
>(
    (_, e1, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, e2, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Or(Box::new(e1), Box::new(e2))
}

fn __action7<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action8<
>(
    (_, a1, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a2, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Equals(Box::new(a1), Box::new(a2))
}

fn __action9<
>(
    (_, a1, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a2, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::NotEquals(Box::new(a1), Box::new(a2))
}

fn __action10<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action11<
>(
    (_, r, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Less(Box::new(r), Box::new(a))
}

fn __action12<
>(
    (_, r, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::LessEqual(Box::new(r), Box::new(a))
}

fn __action13<
>(
    (_, r, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::More(Box::new(r), Box::new(a))
}

fn __action14<
>(
    (_, r, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, a, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::MoreEqual(Box::new(r), Box::new(a))
}

fn __action15<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action16<
>(
    (_, a, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, m, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Add(Box::new(a), Box::new(m))
}

fn __action17<
>(
    (_, a, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, m, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Sub(Box::new(a), Box::new(m))
}

fn __action18<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action19<
>(
    (_, m, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, n, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Mul(Box::new(m), Box::new(n))
}

fn __action20<
>(
    (_, m, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
    (_, n, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Div(Box::new(m), Box::new(n))
}

fn __action21<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, p, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    ast::Primitive::Negation(Box::new(p))
}

fn __action22<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ast::Primitive
{
    __0
}

fn __action23<
>(
    (_, __0, _): (usize, ast::Literal, usize),
) -> ast::Primitive
{
    ast::Primitive::Literal(Box::new(__0))
}

fn __action24<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Primitive
{
    ast::Primitive::Variable(Box::new(ast::Variable {identifier: __0}))
}

fn __action25<
>(
    (_, _, _): (usize, lexer::Token, usize),
    (_, e, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Primitive
{
    e
}

fn __action26<
>(
    (_, __0, _): (usize, i64, usize),
) -> ast::Literal
{
    ast::Literal::Int(__0)
}

fn __action27<
>(
    (_, __0, _): (usize, f64, usize),
) -> ast::Literal
{
    ast::Literal::Float(__0)
}

fn __action28<
>(
    (_, __0, _): (usize, String, usize),
) -> ast::Literal
{
    ast::Literal::String(__0)
}

fn __action29<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
) -> ::std::vec::Vec<ast::Primitive>
{
    vec![__0]
}

fn __action30<
>(
    (_, v, _): (usize, ::std::vec::Vec<ast::Primitive>, usize),
    (_, e, _): (usize, ast::Primitive, usize),
) -> ::std::vec::Vec<ast::Primitive>
{
    { let mut v = v; v.push(e); v }
}

fn __action31<
>(
    (_, __0, _): (usize, ast::Primitive, usize),
    (_, _, _): (usize, lexer::Token, usize),
) -> ast::Primitive
{
    (__0)
}

fn __action32<
>(
    __0: (usize, ast::Primitive, usize),
    __1: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<ast::Primitive>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action31(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        __temp0,
    )
}

fn __action33<
>(
    __0: (usize, ::std::vec::Vec<ast::Primitive>, usize),
    __1: (usize, ast::Primitive, usize),
    __2: (usize, lexer::Token, usize),
) -> ::std::vec::Vec<ast::Primitive>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action31(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action30(
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
