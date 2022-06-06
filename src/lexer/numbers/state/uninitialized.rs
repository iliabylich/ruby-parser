use std::ops::ControlFlow;

use crate::{
    lexer::{
        buffer::{scan_while_matches_pattern, Buffer},
        numbers::{
            state::{integer_prefix::*, Integer, IntegerPrefix, State},
            ExtendNumber, Number,
        },
    },
    token::TokenValue,
};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Uninitialized;

impl ExtendNumber for Uninitialized {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        let byte = buffer.current_byte().unwrap();

        if byte == b'0' {
            buffer.skip_byte();
            number.end += 1;

            match buffer.byte_at(start + 1) {
                Some(b'x' | b'X') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.state = State::IntegerPrefix(IntegerPrefix::Hexadecimal(Hexadecimal));
                    return ControlFlow::Continue(());
                }
                Some(b'b' | b'B') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.state = State::IntegerPrefix(IntegerPrefix::Binary(Binary));
                    return ControlFlow::Continue(());
                }
                Some(b'd' | b'D') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.state = State::IntegerPrefix(IntegerPrefix::Decimal(Decimal));
                    return ControlFlow::Continue(());
                }
                Some(b'_' | b'o' | b'O' | b'0'..=b'7') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.state = State::IntegerPrefix(IntegerPrefix::Octal(Octal));
                    return ControlFlow::Continue(());
                }
                Some(b'8'..=b'9') => {
                    // TODO: report an error here
                    buffer.skip_byte();
                    number.end += 1;

                    let end = scan_while_matches_pattern!(buffer, start + 2, b'_' | b'0'..=b'9')
                        .unwrap_or(0);

                    number.end = end;
                    buffer.set_pos(end);
                    number.state = State::Integer(Integer);
                    return ControlFlow::Break(());
                }

                _other => {
                    // Sole "0" digit
                    number.state = State::Integer(Integer);
                    return ControlFlow::Break(());
                }
            }
        }

        // Definitely a decimal prefix
        number.state = State::IntegerPrefix(IntegerPrefix::Decimal(Decimal));
        ControlFlow::Continue(())
    }
}

impl Into<TokenValue> for Uninitialized {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made no transition")
    }
}
