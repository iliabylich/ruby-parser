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
    string_content::StringContent,
    token::token,
};

#[must_use]
pub(crate) fn handle_escape<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
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
            Escape::SlashU(SlashU::Wide { codepoints, length }) => {
                let codepoints = codepoints.into_iter().collect::<String>().into_bytes();
                escape_content = StringContent::from(codepoints);
                escape_length = length;
            }
            Escape::SlashU(SlashU::Short { codepoint, length }) => {
                escape_content = StringContent::from(codepoint);
                escape_length = length;
            }
            Escape::SlashOctal(SlashOctal { codepoint, length })
            | Escape::SlashX(SlashX { codepoint, length })
            | Escape::SlashMetaCtrl(SlashMetaCtrl { codepoint, length })
            | Escape::SlashByte(SlashByte { codepoint, length }) => {
                escape_content = StringContent::from(codepoint);
                escape_length = length;
            }
        },

        Err(err) => {
            // in case of error:
            // 1. record all valid codepoints (if any)
            let codepoints;

            match &err {
                EscapeError::SlashUError(SlashUError {
                    codepoints: Some(captured_codepoints),
                    ..
                }) => {
                    codepoints = captured_codepoints
                        .iter()
                        .map(|c| *c)
                        .collect::<String>()
                        .into_bytes();
                }
                _ => codepoints = vec![],
            }

            match &err {
                EscapeError::SlashUError(SlashUError {
                    codepoints,
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

            escape_content = StringContent::from(codepoints);

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
            tSTRING_CONTENT(escape_content),
            loc!(start, start + escape_length)
        ),
    })
}
