use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{action::StringExtendAction, types::Interpolation},
    },
    token::token,
};

pub(crate) fn handle_interpolation_end(
    interpolation: &mut Interpolation,
    buffer: &mut Buffer,
    current_curly_nest: usize,
) -> ControlFlow<StringExtendAction> {
    match interpolation {
        Interpolation {
            enabled,
            curly_nest,
        } if *enabled => {
            if buffer.current_byte() == Some(b'}') && *curly_nest == current_curly_nest {
                // Close interpolation
                let token = token!(tSTRING_DEND, buffer.pos(), buffer.pos() + 1);
                buffer.skip_byte();
                *enabled = false;
                return ControlFlow::Break(StringExtendAction::EmitToken { token });
            }

            // yield control to lexer to read interpolated tokens
            return ControlFlow::Break(StringExtendAction::ReadInterpolatedContent);
        }
        _ => {}
    }

    ControlFlow::Continue(())
}
