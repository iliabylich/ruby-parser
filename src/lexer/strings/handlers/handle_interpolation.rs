use std::ops::ControlFlow;

use crate::{
    lexer::{
        atmark::{lookahead_atmark, LookaheadAtMarkResult},
        buffer::Buffer,
        gvar::{lookahead_gvar, LookaheadGvarResult},
        strings::{
            action::StringExtendAction, handlers::handle_processed_string_content,
            types::HasInterpolation,
        },
    },
    token::token,
};

pub(crate) fn handle_interpolation<T>(
    literal: &mut T,
    buffer: &mut Buffer,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: HasInterpolation,
{
    // handle #{ interpolation
    handle_regular_interpolation(literal, buffer, start)?;

    // handle "#@foo" / "#@@foo" interpolation
    handle_raw_ivar_or_cvar_interpolation(buffer, start)?;

    // handle "#$foo" interpolation
    handle_raw_gvar_interpolation(buffer, start)?;

    ControlFlow::Continue(())
}

#[must_use]
fn handle_regular_interpolation<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: HasInterpolation,
{
    if buffer.lookahead(b"#{") {
        handle_processed_string_content(start, buffer.pos())?;

        let token = token!(tSTRING_DBEG, buffer.pos(), buffer.pos() + 2);
        // consume `#{`
        buffer.set_pos(token.loc().end());
        // start interpolation
        literal.interpolation_mut().enabled = true;

        return ControlFlow::Break(StringExtendAction::FoundInterpolation { token });
    }

    ControlFlow::Continue(())
}

#[must_use]
fn handle_raw_ivar_or_cvar_interpolation(
    buffer: &mut Buffer,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.lookahead(b"#@") {
        handle_processed_string_content(start, buffer.pos())?;

        // here we (possibly) handle only `#` of "#@foo" / "#@@foo" interpolation
        if let LookaheadAtMarkResult::Ok(_) = lookahead_atmark(buffer, buffer.pos() + 1) {
            let token = token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1);

            // consume `#`
            buffer.set_pos(token.loc().end());

            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    if buffer.pos() > 0
        && buffer.byte_at(buffer.pos() - 1) == Some(b'#')
        && buffer.current_byte() == Some(b'@')
    {
        // here we (possibly) have already dipsatched `#` of "#@foo" / "#@@foo" interpolation
        if let LookaheadAtMarkResult::Ok(token) = lookahead_atmark(buffer, buffer.pos()) {
            // consume variable
            buffer.set_pos(token.loc().end());
            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    ControlFlow::Continue(())
}

#[must_use]
fn handle_raw_gvar_interpolation(
    buffer: &mut Buffer,
    start: usize,
) -> ControlFlow<StringExtendAction> {
    if buffer.lookahead(b"#$") {
        handle_processed_string_content(start, buffer.pos())?;

        // here we (possibly) handle only `#` of "#$foo" interpolation
        if let LookaheadGvarResult::Ok(_) = lookahead_gvar(buffer, buffer.pos() + 1) {
            let token = token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1);

            // consume `#`
            buffer.set_pos(token.loc().end());

            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    if buffer.pos() > 0
        && buffer.byte_at(buffer.pos() - 1) == Some(b'#')
        && buffer.current_byte() == Some(b'$')
    {
        // here we (possibly) have already dipsatched `#` of "#@foo" / "#@@foo" interpolation
        if let LookaheadGvarResult::Ok(token) = lookahead_gvar(buffer, buffer.pos()) {
            // consume variable
            buffer.set_pos(token.loc().end());
            return ControlFlow::Break(StringExtendAction::EmitToken { token });
        }
    }

    ControlFlow::Continue(())
}
