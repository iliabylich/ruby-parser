use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{BufferWithCursor, Lookahead},
        strings::{
            action::StringExtendAction,
            escapes::{
                Escape, EscapeError, SlashByte, SlashByteError, SlashMetaCtrl, SlashMetaCtrlError,
                SlashOctal, SlashU, SlashUError, SlashUPerCodepointError, SlashX, SlashXError,
            },
            handlers::handle_processed_string_content,
        },
    },
    loc::loc,
    token::token,
};

#[must_use]
pub(crate) fn handle_escape(
    buffer: &mut BufferWithCursor,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    let escape_content;
    let escape_length;

    if buffer.current_byte() == Some(b'\\') {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;
    }

    match Escape::lookahead(buffer.for_lookahead(), buffer.pos()) {
        Ok(None) => {
            return ControlFlow::Continue(());
        }

        Ok(Some(escape)) => match escape {
            Escape::SlashU(SlashU::Wide { bytes, length }) => {
                escape_content = bytes;
                escape_length = length;
            }
            Escape::SlashU(SlashU::Short { bytes, length }) => {
                escape_content = bytes;
                escape_length = length;
            }
            Escape::SlashOctal(SlashOctal {
                byte: codepoint,
                length,
            })
            | Escape::SlashX(SlashX {
                byte: codepoint,
                length,
            })
            | Escape::SlashMetaCtrl(SlashMetaCtrl {
                byte: codepoint,
                length,
            })
            | Escape::SlashByte(SlashByte {
                byte: codepoint,
                length,
            }) => {
                escape_content = vec![codepoint];
                escape_length = length;
            }
        },

        Err(err) => {
            // in case of error:
            // 1. record all valid codepoints (if any)
            let valid_codepoints;

            match &err {
                EscapeError::SlashUError(SlashUError {
                    valid_bytes: Some(captured_codepoints),
                    ..
                }) => {
                    valid_codepoints = captured_codepoints.clone();
                }
                _ => valid_codepoints = vec![],
            }

            match &err {
                EscapeError::SlashUError(SlashUError {
                    valid_bytes: codepoints,
                    errors,
                    length,
                }) => {
                    eprintln!("Got \\u errors: {:?} {:?} {:?}", codepoints, errors, length);
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

            escape_content = valid_codepoints;

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
