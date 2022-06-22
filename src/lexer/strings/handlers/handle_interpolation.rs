use std::ops::ControlFlow;

use crate::{
    lexer::{
        atmark::AtMark,
        buffer::{BufferWithCursor, Lookahead},
        gvar::Gvar,
        strings::{
            action::StringExtendAction, handlers::handle_processed_string_content,
            types::Interpolation,
        },
    },
    token::token,
};

pub(crate) fn handle_interpolation<'a>(
    interpolation: &mut Interpolation,
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    // handle #{ interpolation
    handle_regular_interpolation(interpolation, buffer, start)?;

    // handle "#@foo" / "#@@foo" interpolation
    handle_raw_ivar_or_cvar_interpolation(buffer, start)?;

    // handle "#$foo" interpolation
    handle_raw_gvar_interpolation(buffer, start)?;

    ControlFlow::Continue(())
}

#[must_use]
fn handle_regular_interpolation<'a>(
    interpolation: &mut Interpolation,
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    if buffer.lookahead(b"#{") {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        let token = token!(tSTRING_DBEG, buffer.pos(), buffer.pos() + 2);
        // consume `#{`
        buffer.set_pos(token.loc().end);
        // start interpolation
        interpolation.enabled = true;

        return ControlFlow::Break(StringExtendAction::FoundInterpolation { token });
    }

    ControlFlow::Continue(())
}

#[must_use]
fn handle_raw_ivar_or_cvar_interpolation<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    if buffer.lookahead(b"#@") {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        // here we (possibly) handle only `#` of "#@foo" / "#@@foo" interpolation
        if let Ok(_) = AtMark::lookahead(buffer.for_lookahead(), buffer.pos() + 1) {
            let token = token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1);

            // consume `#`
            buffer.set_pos(token.loc().end);

            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    if buffer.pos() > 0
        && buffer.byte_at(buffer.pos() - 1) == Some(b'#')
        && buffer.current_byte() == Some(b'@')
    {
        // here we (possibly) have already dipsatched `#` of "#@foo" / "#@@foo" interpolation
        if let Ok(AtMark { token }) = AtMark::lookahead(buffer.for_lookahead(), buffer.pos()) {
            // consume variable
            buffer.set_pos(token.loc().end);
            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    ControlFlow::Continue(())
}

#[must_use]
fn handle_raw_gvar_interpolation<'a>(
    buffer: &mut BufferWithCursor<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction<'a>> {
    if buffer.lookahead(b"#$") {
        handle_processed_string_content(buffer.for_lookahead(), start, buffer.pos())?;

        // here we (possibly) handle only `#` of "#$foo" interpolation
        if let Ok(_) = Gvar::lookahead(buffer.for_lookahead(), buffer.pos() + 1) {
            let token = token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1);

            // consume `#`
            buffer.set_pos(token.loc().end);

            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    if buffer.pos() > 0
        && buffer.byte_at(buffer.pos() - 1) == Some(b'#')
        && buffer.current_byte() == Some(b'$')
    {
        // here we (possibly) have already dipsatched `#` of "#@foo" / "#@@foo" interpolation
        if let Ok(Gvar { token }) = Gvar::lookahead(buffer.for_lookahead(), buffer.pos()) {
            // consume variable
            buffer.set_pos(token.loc().end);
            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    ControlFlow::Continue(())
}
