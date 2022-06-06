use std::ops::ControlFlow;

use crate::lexer::buffer::{scan_while_matches_pattern, Buffer};
use crate::token::{Loc, Token, TokenValue};

pub(crate) mod scan;
pub(crate) mod try_to_extend_with;

#[derive(Clone, Copy, Debug)]
struct Uninitialized;

#[derive(Clone, Copy, Debug)]
struct HexadecimalPrefix;
#[derive(Clone, Copy, Debug)]
struct BinaryPrefix;
#[derive(Clone, Copy, Debug)]
struct OctalPrefix;
#[derive(Clone, Copy, Debug)]
struct DecimalPrefix;

#[derive(Clone, Copy, Debug)]
struct Integer;
#[derive(Clone, Copy, Debug)]
struct Rational;
#[derive(Clone, Copy, Debug)]
struct Imaginary;
#[derive(Clone, Copy, Debug)]
struct FloatWithDotNumber;
#[derive(Clone, Copy, Debug)]
struct FloatWithESuffix;

#[derive(Clone, Copy, Debug)]
enum NumberKind {
    Uninitialized(Uninitialized),

    HexadecimalPrefix(HexadecimalPrefix),
    BinaryPrefix(BinaryPrefix),
    OctalPrefix(OctalPrefix),
    DecimalPrefix(DecimalPrefix),

    Integer(Integer),
    Rational(Rational),
    Imaginary(Imaginary),
    FloatWithDotNumber(FloatWithDotNumber),
    FloatWithESuffix(FloatWithESuffix),
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Number {
    kind: NumberKind,
    begin: usize,
    end: usize,
}

impl Number {
    fn new(start: usize) -> Self {
        Self {
            kind: NumberKind::Uninitialized(Uninitialized),
            begin: start,
            end: start,
        }
    }
}

trait ExtendNumber {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()>;
}

macro_rules! try_sub_parser {
    ($fn:expr, $buffer:expr, $start:expr, $number:expr) => {
        if let Some(len) = $fn($buffer, $start) {
            $buffer.set_pos($buffer.pos() + len.get());

            $number.end += len.get();
            true
        } else {
            false
        }
    };
}

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
                    number.kind = NumberKind::HexadecimalPrefix(HexadecimalPrefix);
                    return ControlFlow::Continue(());
                }
                Some(b'b' | b'B') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.kind = NumberKind::BinaryPrefix(BinaryPrefix);
                    return ControlFlow::Continue(());
                }
                Some(b'd' | b'D') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.kind = NumberKind::DecimalPrefix(DecimalPrefix);
                    return ControlFlow::Continue(());
                }
                Some(b'_' | b'o' | b'O' | b'0'..=b'7') => {
                    buffer.skip_byte();
                    number.end += 1;
                    number.kind = NumberKind::OctalPrefix(OctalPrefix);
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
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Break(());
                }

                _other => {
                    // Sole "0" digit
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Break(());
                }
            }
        }

        // Definitely a decimal prefix
        number.kind = NumberKind::DecimalPrefix(DecimalPrefix);
        ControlFlow::Continue(())
    }
}

impl Into<TokenValue> for Uninitialized {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made no transition")
    }
}

// Runs after consuming `0x` hexadecimal prefix
impl ExtendNumber for HexadecimalPrefix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::hexadecimal, buffer, start, number) {
            number.kind = NumberKind::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

impl Into<TokenValue> for HexadecimalPrefix {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made an incomplete transition to {:?}", self)
    }
}

// Runs after consuming `0b` binary prefix
impl ExtendNumber for BinaryPrefix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::binary, buffer, start, number) {
            number.kind = NumberKind::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

impl Into<TokenValue> for BinaryPrefix {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made an incomplete transition to {:?}", self)
    }
}

// Runs after consuming octal prefix (`0`)
impl ExtendNumber for OctalPrefix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::octal, buffer, start, number) {
            number.kind = NumberKind::Integer(Integer);
            ControlFlow::Continue(())
        } else {
            panic!("numeric literal without digits")
        }
    }
}

impl Into<TokenValue> for OctalPrefix {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made an incomplete transition to {:?}", self)
    }
}

// Runs after consuming decimal prefix (`0d` or no prefix)
impl ExtendNumber for DecimalPrefix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(scan::decimal, buffer, start, number) {
            number.kind = NumberKind::Integer(Integer);
            return ControlFlow::Continue(());
        } else {
            panic!("numeric literal without digits")
        }
    }
}

impl Into<TokenValue> for DecimalPrefix {
    fn into(self) -> TokenValue {
        unreachable!("ExtendNumber made an incomplete transition to {:?}", self)
    }
}

impl ExtendNumber for Integer {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::dot_number_suffix, buffer, start, number) {
            number.kind = NumberKind::FloatWithDotNumber(FloatWithDotNumber);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::e_suffix, buffer, start, number) {
            number.kind = NumberKind::FloatWithESuffix(FloatWithESuffix);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::r_suffix, buffer, start, number) {
            number.kind = NumberKind::Rational(Rational);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.kind = NumberKind::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for Integer {
    fn into(self) -> TokenValue {
        TokenValue::tINTEGER
    }
}

impl ExtendNumber for Rational {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.kind = NumberKind::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for Rational {
    fn into(self) -> TokenValue {
        TokenValue::tRATIONAL
    }
}

impl ExtendNumber for Imaginary {
    fn extend(_number: &mut Number, _buffer: &mut Buffer) -> ControlFlow<()> {
        // Imaginary numbers can't be extended to anything bigger
        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for Imaginary {
    fn into(self) -> TokenValue {
        TokenValue::tIMAGINARY
    }
}

impl ExtendNumber for FloatWithDotNumber {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::e_suffix, buffer, start, number) {
            number.kind = NumberKind::FloatWithESuffix(FloatWithESuffix);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::r_suffix, buffer, start, number) {
            number.kind = NumberKind::Rational(Rational);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.kind = NumberKind::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for FloatWithDotNumber {
    fn into(self) -> TokenValue {
        TokenValue::tFLOAT
    }
}

impl ExtendNumber for FloatWithESuffix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        let start = buffer.pos();

        if try_sub_parser!(try_to_extend_with::r_suffix, buffer, start, number) {
            number.kind = NumberKind::Rational(Rational);
            return ControlFlow::Continue(());
        }

        if try_sub_parser!(try_to_extend_with::i_suffix, buffer, start, number) {
            number.kind = NumberKind::Imaginary(Imaginary);
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl Into<TokenValue> for FloatWithESuffix {
    fn into(self) -> TokenValue {
        TokenValue::tFLOAT
    }
}

impl ExtendNumber for Number {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        match number.kind {
            NumberKind::Uninitialized(_) => Uninitialized::extend(number, buffer),
            NumberKind::HexadecimalPrefix(_) => HexadecimalPrefix::extend(number, buffer),
            NumberKind::BinaryPrefix(_) => BinaryPrefix::extend(number, buffer),
            NumberKind::OctalPrefix(_) => OctalPrefix::extend(number, buffer),
            NumberKind::DecimalPrefix(_) => DecimalPrefix::extend(number, buffer),
            NumberKind::Integer(_) => Integer::extend(number, buffer),
            NumberKind::Rational(_) => Rational::extend(number, buffer),
            NumberKind::Imaginary(_) => Imaginary::extend(number, buffer),
            NumberKind::FloatWithDotNumber(_) => FloatWithDotNumber::extend(number, buffer),
            NumberKind::FloatWithESuffix(_) => FloatWithESuffix::extend(number, buffer),
        }
    }
}

impl Into<TokenValue> for Number {
    fn into(self) -> TokenValue {
        match self.kind {
            NumberKind::Uninitialized(inner) => inner.into(),
            NumberKind::HexadecimalPrefix(inner) => inner.into(),
            NumberKind::BinaryPrefix(inner) => inner.into(),
            NumberKind::OctalPrefix(inner) => inner.into(),
            NumberKind::DecimalPrefix(inner) => inner.into(),
            NumberKind::Integer(inner) => inner.into(),
            NumberKind::Rational(inner) => inner.into(),
            NumberKind::Imaginary(inner) => inner.into(),
            NumberKind::FloatWithDotNumber(inner) => inner.into(),
            NumberKind::FloatWithESuffix(inner) => inner.into(),
        }
    }
}

impl Into<Token> for Number {
    fn into(self) -> Token {
        Token(self.into(), Loc(self.begin, self.end))
    }
}

pub(crate) fn parse_number<'a>(buffer: &mut Buffer<'a>) -> Token {
    let mut number = Number::new(buffer.pos());

    loop {
        let action = Number::extend(&mut number, buffer);

        if action == ControlFlow::Break(()) {
            break;
        }
    }

    let token = number.into();
    println!("{:?}", token);
    token
}

#[cfg(test)]
mod tests;
