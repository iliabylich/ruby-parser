macro_rules! assert_emits_interpolation_end {
    ($literal:expr) => {
        assert_emits_1_token_and_then_eof!(
            test = test_interpolation_end,
            literal = $literal,
            input = b"}",
            token = token!(tSTRING_DEND, loc!(0, 1)),
            pre = |literal: &mut StringLiteral| {
                match literal {
                    StringLiteral::StringInterp(string) => string.enable_interpolation(),
                    StringLiteral::Regexp(regexp) => regexp.enable_interpolation(),
                    _ => panic!("String literal {:?} doesn't embed Interpolation", literal),
                };
            }
        );
    };
}
pub(crate) use assert_emits_interpolation_end;

macro_rules! assert_ignores_interpolation_end {
    ($literal:expr) => {
        assert_emits_1_token_and_then_eof!(
            test = test_interpolation_end,
            literal = $literal,
            input = b"}",
            token = token!(tSTRING_CONTENT, loc!(0, 1)),
            pre = |literal: &mut StringLiteral| {
                match literal {
                    StringLiteral::StringPlain(_) => {}
                    _ => panic!(
                        "{:?} DOES support Interpolation, the test makes no sense",
                        literal
                    ),
                };
            }
        );
    };
}
pub(crate) use assert_ignores_interpolation_end;
