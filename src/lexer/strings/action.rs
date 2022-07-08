use crate::token::Token;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction {
    EmitToken { token: Token },
    FoundStringEnd { token: Token },
    FoundInterpolation { token: Token },
    EmitEOF { at: usize },
    ReadInterpolatedContent,
}
