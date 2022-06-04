use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::Buffer,
        strings::{action::StringExtendAction, types::HasInterpolation},
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
    if literal.supports_interpolation() && literal.currently_in_interpolation() {
        if buffer.current_byte() == Some(b'}')
            && literal.interpolation_started_with_curly_level() == current_curly_nest
        {
            // Close interpolation
            let token = token!(tSTRING_DEND, buffer.pos(), buffer.pos() + 1);
            buffer.skip_byte();
            *literal.currently_in_interpolation_mut() = false;
            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }

        // yield control to lexer to read interpolated tokens
        return ControlFlow::Break(StringExtendAction::ReadInterpolatedContent);
    }

    ControlFlow::Continue(())
}
