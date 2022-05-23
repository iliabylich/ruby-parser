pub(crate) mod literal;
pub(crate) mod stack;

use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};
use literal::{StringExtendAction, StringLiteral};

#[derive(Debug)]
pub(crate) enum ParseStringResult<'a> {
    ReadInterpolatedContent,
    EmitToken { token: Token<'a> },
    CloseLiteral { end_token: Token<'a> },
    EmitEOF,
}

pub(crate) fn parse_string<'a>(
    literal: &mut StringLiteral<'a>,
    buffer: &mut Buffer<'a>,
    curly_nest: usize,
) -> ParseStringResult<'a> {
    if literal.is_currently_in_interpolation() {
        if literal.can_close_interpolation(buffer, curly_nest) {
            // Close interpolation
            let token = Token(
                TokenValue::tSTRING_DEND,
                Loc(buffer.pos(), buffer.pos() + 1),
            );
            buffer.skip_byte();
            literal.currently_in_interpolation = false;
            return ParseStringResult::EmitToken { token };
        }

        // yield control to lexer to read interpolated tokens
        return ParseStringResult::ReadInterpolatedContent;
    }

    let extend_action = literal.extend(buffer);
    match extend_action {
        StringExtendAction::EmitToken { token } => ParseStringResult::EmitToken { token },
        StringExtendAction::FoundStringEnd { token } => {
            ParseStringResult::CloseLiteral { end_token: token }
        }
        StringExtendAction::FoundInterpolation { token } => {
            literal.currently_in_interpolation = true;
            ParseStringResult::EmitToken { token }
        }
        StringExtendAction::NoAction => ParseStringResult::EmitEOF,
    }
}

#[cfg(test)]
mod tests;
