use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{Buffer, Lookahead},
        string_content::StringContent,
        strings::{
            action::StringExtendAction,
            escapes::{LooakeadhSlashUResult, SlashU},
        },
    },
    token::token,
};

#[must_use]
pub(crate) fn handle_slash_u<'a>(
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    let (string_content, length) = match SlashU::lookahead(buffer, start) {
        LooakeadhSlashUResult::Short { codepoint, length } => {
            (StringContent::from(codepoint), length)
        }
        LooakeadhSlashUResult::Wide { codepoints, length } => {
            let codepoints = codepoints
                .into_iter()
                .cloned()
                .collect::<String>()
                .into_bytes();

            (StringContent::from(codepoints), length)
        }
        LooakeadhSlashUResult::Nothing => {
            return ControlFlow::Continue(());
        }
        LooakeadhSlashUResult::Err {
            codepoints,
            errors,
            length,
        } => {
            let codepoints = if let Some(codepoints) = codepoints {
                codepoints
                    .into_iter()
                    .cloned()
                    .collect::<String>()
                    .into_bytes()
            } else {
                vec![]
            };

            if !errors.is_empty() {
                panic!("report errors {:?}", errors);
            }

            (StringContent::from(codepoints), length)
        }
    };

    buffer.set_pos(start + length);
    ControlFlow::Break(StringExtendAction::EmitToken {
        token: token!(tSTRING_CONTENT(string_content), start, start + length),
    })
}
