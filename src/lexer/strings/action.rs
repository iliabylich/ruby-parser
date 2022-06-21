use crate::token::Token;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction<'a> {
    EmitToken { token: Token<'a> },
    FoundStringEnd { token: Token<'a> },
    FoundInterpolation { token: Token<'a> },
    EmitEOF { at: usize },
    ReadInterpolatedContent,
}
