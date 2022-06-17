use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{BufferWithCursor, Lookahead},
        string_content::StringContent,
        strings::{
            action::StringExtendAction,
            escapes::{
                Escape, EscapeError, SlashMetaCtrl, SlashMetaCtrlError, SlashOctal,
                SlashOctalError, SlashU, SlashUError, SlashX, SlashXError,
            },
        },
    },
    token::token,
};

#[must_use]
pub(crate) fn handle_escape<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    let string_content;
    let escape_length;

    match Escape::lookahead(buffer.for_lookahead(), start) {
        Ok(None) => {
            return ControlFlow::Continue(());
        }

        Ok(Some(escape)) => match escape {
            Escape::SlashU(SlashU::Wide { codepoints, length }) => {
                let codepoints = codepoints
                    .into_iter()
                    .cloned()
                    .collect::<String>()
                    .into_bytes();
                string_content = StringContent::from(codepoints);
                escape_length = length;
            }
            Escape::SlashU(SlashU::Short { codepoint, length }) => {
                string_content = StringContent::from(codepoint);
                escape_length = length;
            }
            Escape::SlashOctal(SlashOctal { codepoint, length })
            | Escape::SlashX(SlashX { codepoint, length })
            | Escape::SlashMetaCtrl(SlashMetaCtrl { codepoint, length }) => {
                string_content = StringContent::from(codepoint);
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
                        .cloned()
                        .collect::<String>()
                        .into_bytes();
                }
                _ => codepoints = vec![],
            }
            // 2. TODO: report `err`

            string_content = StringContent::from(codepoints);

            escape_length = match err {
                EscapeError::SlashUError(SlashUError { length, .. })
                | EscapeError::SlashOctalError(SlashOctalError { length })
                | EscapeError::SlashXError(SlashXError { length })
                | EscapeError::SlashMetaCtrlError(SlashMetaCtrlError { length }) => length,
            };
        }
    };

    buffer.set_pos(start + escape_length);
    ControlFlow::Break(StringExtendAction::EmitToken {
        token: token!(
            tSTRING_CONTENT(string_content),
            start,
            start + escape_length
        ),
    })
}
