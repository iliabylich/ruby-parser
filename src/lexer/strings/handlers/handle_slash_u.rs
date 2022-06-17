use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{BufferWithCursor, Lookahead},
        string_content::StringContent,
        strings::{
            action::StringExtendAction,
            escapes::{SlashU, SlashUError},
        },
    },
    token::token,
};

#[must_use]
pub(crate) fn handle_slash_u<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    let (string_content, length) = match SlashU::lookahead(buffer.for_lookahead(), start) {
        Ok(Some(SlashU::Short { codepoint, length })) => (StringContent::from(codepoint), length),
        Ok(Some(SlashU::Wide { codepoints, length })) => {
            let codepoints = codepoints
                .into_iter()
                .cloned()
                .collect::<String>()
                .into_bytes();

            (StringContent::from(codepoints), length)
        }
        Ok(None) => {
            return ControlFlow::Continue(());
        }
        Err(SlashUError {
            codepoints,
            errors,
            length,
        }) => {
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
