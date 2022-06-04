use crate::token::Token;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StringExtendAction {
    EmitToken { token: Token },
    FoundStringEnd { token: Token },
    FoundInterpolation { token: Token },
    EmitEOF,
    ReadInterpolatedContent,
}
