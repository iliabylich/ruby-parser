use std::ops::ControlFlow;

use crate::{
    lexer::{
        atmark::{lookahead_atmark, LookaheadAtMarkResult},
        buffer::Buffer,
        gvar::{lookahead_gvar, LookaheadGvarResult},
        strings::{
            action::StringExtendAction, handlers::string_content_to_emit,
            types::StringLiteralAttributes,
        },
    },
    token::token,
};

pub(crate) fn handle_interpolation<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    if buffer.const_lookahead(b"#{") {
        // handle #{ interpolation
        handle_common_interpolation(literal, buffer, start)?;
    }

    if buffer.const_lookahead(b"#@") {
        // handle #@foo / #@@foo interpolation
        handle_raw_atmark_interpolation(literal, buffer, start)?;
    }

    if buffer.const_lookahead(b"#$") {
        // handle #$foo interpolation
        handle_raw_gvar_interpolation(literal, buffer, start)?;
    }

    ControlFlow::Continue(())
}

#[must_use]
fn handle_common_interpolation<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    // #{ interpolation
    let action = StringExtendAction::FoundInterpolation {
        token: token!(tSTRING_DBEG, buffer.pos(), buffer.pos() + 2),
    };
    let string_content = string_content_to_emit(start, buffer.pos());
    buffer.set_pos(buffer.pos() + 2);

    if let Some(token) = string_content {
        literal.next_action_mut().add(action);
        ControlFlow::Break(StringExtendAction::EmitToken { token })
    } else {
        ControlFlow::Break(action)
    }
}

#[must_use]
fn handle_raw_atmark_interpolation<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    if let LookaheadAtMarkResult::Ok(token) = lookahead_atmark(buffer, buffer.pos() + 1) {
        // #@foo interpolation
        let interp_action = StringExtendAction::EmitToken {
            token: token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1),
        };
        let var_action = StringExtendAction::EmitToken { token };
        let string_content = string_content_to_emit(start, buffer.pos());
        buffer.set_pos(token.loc().end());

        if let Some(token) = string_content {
            literal.next_action_mut().add(interp_action);
            literal.next_action_mut().add(var_action);
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            literal.next_action_mut().add(var_action);
            ControlFlow::Break(interp_action)
        }
    } else {
        // just #@ string content without subsequent identifier
        // keep reading
        ControlFlow::Continue(())
    }
}

#[must_use]
fn handle_raw_gvar_interpolation<'a, T>(
    literal: &mut T,
    buffer: &mut Buffer<'a>,
    start: usize,
) -> ControlFlow<StringExtendAction>
where
    T: StringLiteralAttributes<'a>,
{
    if let LookaheadGvarResult::Ok(token) = lookahead_gvar(buffer, buffer.pos() + 1) {
        // #$foo interpolation
        let interp_action = StringExtendAction::EmitToken {
            token: token!(tSTRING_DVAR, buffer.pos(), buffer.pos() + 1),
        };
        let var_action = StringExtendAction::EmitToken { token };
        let string_content = string_content_to_emit(start, buffer.pos());
        buffer.set_pos(token.loc().end());

        if let Some(token) = string_content {
            literal.next_action_mut().add(interp_action);
            literal.next_action_mut().add(var_action);
            ControlFlow::Break(StringExtendAction::EmitToken { token })
        } else {
            literal.next_action_mut().add(var_action);
            ControlFlow::Break(interp_action)
        }
    } else {
        // just #$ string content without subsequent identifier
        // keep reading
        ControlFlow::Continue(())
    }
}
