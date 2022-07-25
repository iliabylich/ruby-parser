use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::BufferWithCursor,
        strings::{
            action::StringExtendAction,
            escapes::{
                Escape, EscapeError, SlashByte, SlashByteError, SlashMetaCtrl, SlashMetaCtrlError,
                SlashOctal, SlashU, SlashUError, SlashUPerCodepointError, SlashX, SlashXError,
            },
            handlers::handle_processed_string_content,
        },
    },
    loc::{loc, Loc},
    token::{token, TokenValue},
};

#[must_use]
pub(crate) fn handle_escape(
    buffer: &mut BufferWithCursor,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    let escape_content: TokenValue;
    let escape_length;

    if buffer.current_byte() == Some(b'\\') {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;
    }

    let lookahead_start = buffer.pos();
    match Escape::lookahead(buffer.for_lookahead_mut(), lookahead_start) {
        Ok(None) => {
            return ControlFlow::Continue(());
        }

        Ok(Some(escape)) => match escape {
            Escape::SlashU(SlashU::Wide {
                escaped_loc,
                length,
            }) => {
                escape_content = TokenValue::UnescapedChars {
                    loc: escaped_loc,
                    buffer: buffer.for_lookahead(),
                };
                escape_length = length;
            }
            Escape::SlashU(SlashU::Short { codepoint, length }) => {
                escape_content = TokenValue::from(codepoint);
                escape_length = length;
            }
            Escape::SlashOctal(SlashOctal { byte, length })
            | Escape::SlashX(SlashX { byte, length })
            | Escape::SlashMetaCtrl(SlashMetaCtrl { byte, length })
            | Escape::SlashByte(SlashByte { byte, length }) => {
                escape_content = TokenValue::from(byte);
                escape_length = length;
            }
        },

        Err(err) => {
            // in case of error:
            // 1. record all valid codepoints (if any)
            let escaped_loc;

            match &err {
                EscapeError::SlashUError(SlashUError {
                    escaped_loc: loc, ..
                }) if !loc.is_empty() => {
                    escaped_loc = *loc;
                }
                _ => escaped_loc = Loc { start: 0, end: 0 },
            }

            match &err {
                EscapeError::SlashUError(SlashUError {
                    escaped_loc,
                    errors,
                    length,
                }) => {
                    eprintln!(
                        "Got \\u errors: {:?} {:?} {:?}",
                        escaped_loc, errors, length
                    );
                    for error in errors {
                        match error {
                            SlashUPerCodepointError::Expected4Got { start, length } => {
                                eprintln!("expected 4 got {:?} {:?}", start, length);
                            }
                            SlashUPerCodepointError::TooLong { start, length } => {
                                eprintln!("too long {:?} {:?}", start, length);
                            }
                            SlashUPerCodepointError::NonHex { start, length } => {
                                eprintln!("non-hex {:?} {:?}", start, length);
                            }
                            SlashUPerCodepointError::NoRCurly { start } => {
                                eprintln!("no closing rcurly {:?} {:?}", start, length);
                            }
                        }
                    }
                }
                EscapeError::SlashXError(SlashXError { length }) => {
                    eprintln!("\\x error {:?}", length);
                }
                EscapeError::SlashMetaCtrlError(SlashMetaCtrlError { length }) => {
                    eprintln!("\\ meta/ctrl error {:?}", length);
                }
                EscapeError::SlashByteError(SlashByteError { length }) => {
                    eprintln!("\\byte error {:?}", length);
                }
            }
            // 2. TODO: report `err`

            escape_content = TokenValue::UnescapedChars {
                loc: escaped_loc,
                buffer: buffer.for_lookahead(),
            };

            escape_length = match err {
                EscapeError::SlashUError(SlashUError { length, .. })
                | EscapeError::SlashXError(SlashXError { length })
                | EscapeError::SlashMetaCtrlError(SlashMetaCtrlError { length })
                | EscapeError::SlashByteError(SlashByteError { length }) => length,
            };
        }
    };

    buffer.set_pos(buffer.pos() + escape_length);

    ControlFlow::Break(StringExtendAction::EmitToken {
        token: token!(
            tSTRING_CONTENT,
            loc!(start, start + escape_length),
            escape_content
        ),
    })
}
