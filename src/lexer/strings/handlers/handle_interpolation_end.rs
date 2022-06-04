use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{
            action::StringExtendAction,
            types::{HasInterpolation, Interpolation},
        },
    },
    token::token,
};

pub(crate) fn handle_interpolation_end<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    current_curly_nest: usize,
) -> ControlFlow<StringExtendAction>
where
    T: HasInterpolation,
{
    match literal.interpolation_mut() {
        Interpolation::Available {
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
