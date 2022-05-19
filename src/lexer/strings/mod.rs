pub(crate) mod literal;
pub(crate) mod stack;

use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};
use literal::{StringExtendAction, StringLiteral};

#[derive(Debug)]
pub(crate) enum ParseStringResult<'a> {
    ReadInterpolatedContent {
        interpolation_started_with_curly_level: usize,
    },
    EmitToken {
        token: Token<'a>,
    },
    CloseLiteral {
        end_token: Token<'a>,
    },
}

pub(crate) fn parse_string<'a>(
    literal: &mut StringLiteral<'a>,
    buffer: &mut Buffer<'a>,
) -> ParseStringResult<'a> {
    if literal.currently_in_interpolation && literal.supports_interpolation {
        // yield control to lexer to read interpolated tokens
        return ParseStringResult::ReadInterpolatedContent {
            interpolation_started_with_curly_level: literal.interpolation_started_with_curly_level,
        };
    }

    let start = buffer.pos();

    let extend_action = literal.extend(buffer);
    match extend_action {
        StringExtendAction::FoundStringEnd {
            string_end_starts_at,
        } => {
            // flush what's available (if any)
            if string_end_starts_at > start {
                let token = Token(
                    TokenValue::tSTRING_CONTENT(buffer.slice(start, string_end_starts_at)),
                    Loc(start, string_end_starts_at),
                );
                // Set buffer.pos to tSTRING_END loc that will be recorded
                // on the next run
                buffer.set_pos(string_end_starts_at);
                ParseStringResult::EmitToken { token }
            } else {
                // No string content recorded, just emit tSTRING_END
                let token = Token(
                    TokenValue::tSTRING_END(literal.ends_with),
                    Loc(
                        string_end_starts_at,
                        string_end_starts_at + literal.ends_with.len(),
                    ),
                );
                // Set buffer.pos to post-string location
                buffer.set_pos(string_end_starts_at + literal.ends_with.len());
                ParseStringResult::CloseLiteral { end_token: token }
            }
        }
        StringExtendAction::FoundInterpolation {
            interpolation_starts_at,
        } => {
            // flush what's available (if any)
            if interpolation_starts_at > start {
                let token = Token(
                    TokenValue::tSTRING_CONTENT(buffer.slice(start, interpolation_starts_at)),
                    Loc(start, interpolation_starts_at),
                );
                // Set buffer.pos to tSTRING_END loc that will be recorded
                // on the next run
                buffer.set_pos(interpolation_starts_at);
                ParseStringResult::EmitToken { token }
            } else {
                // No string content recorded
                let token = Token(
                    TokenValue::tSTRING_DBEG(b"#{"),
                    Loc(interpolation_starts_at, interpolation_starts_at + 2),
                );
                literal.currently_in_interpolation = true;
                buffer.set_pos(interpolation_starts_at + 2);
                ParseStringResult::EmitToken { token }
            }
        }
        StringExtendAction::FoundInterpolatedToken { token } => {
            buffer.set_pos(token.loc().end());
            ParseStringResult::EmitToken { token }
        }
        StringExtendAction::FoundEscapedNl {
            escaped_nl_starts_at,
        } => {
            let token = Token(
                TokenValue::tSTRING_CONTENT(buffer.slice(start, escaped_nl_starts_at)),
                Loc(start, escaped_nl_starts_at),
            );
            // Set buffer.pos to tSTRING_END loc that will be recorded
            // on the next run
            buffer.set_pos(escaped_nl_starts_at);
            ParseStringResult::EmitToken { token }
        }
    }
}

#[cfg(test)]
mod tests;
