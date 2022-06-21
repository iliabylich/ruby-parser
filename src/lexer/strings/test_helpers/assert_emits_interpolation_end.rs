macro_rules! assert_emits_interpolation_end {
    ($literal:expr) => {
        assert_emits_extend_action!(
            test = test_interpolation_end,
            literal = $literal,
            input = b"}",
            action = StringExtendAction::EmitToken {
                token: token!(tSTRING_DEND, 0, 1)
            },
            pre = |literal: &mut StringLiteral| {
                match literal {
                    StringLiteral::StringInterp(string) => string.enable_interpolation(),
                    StringLiteral::Regexp(regexp) => regexp.enable_interpolation(),
                    _ => panic!("String literal {:?} doesn't embed Interpolation", literal),
                };
            },
            post = |action: StringExtendAction| {
                assert_eq!(
                    action,
                    StringExtendAction::EmitEOF { at: 1 },
                    "2nd action daction doesn't match"
                )
            }
        );
    };
}
pub(crate) use assert_emits_interpolation_end;
