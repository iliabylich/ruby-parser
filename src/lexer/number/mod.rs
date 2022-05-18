use crate::lexer::buffer::Buffer;
use crate::token::{Loc, Token, TokenValue};

mod read;
mod try_to_extend_with;

#[derive(Clone, Copy, Debug)]
struct Uninitialized;
#[derive(Clone, Copy, Debug)]
struct Integer;
#[derive(Clone, Copy, Debug)]
struct Rational;
#[derive(Clone, Copy, Debug)]
struct Imaginary;
#[derive(Clone, Copy, Debug)]
struct Float {
    has_dot_number_suffix: bool,
    has_e_suffix: bool,
}

#[derive(Clone, Copy, Debug)]
enum NumberKind {
    Uninitialized(Uninitialized),
    Integer(Integer),
    Rational(Rational),
    Imaginary(Imaginary),
    Float(Float),
}

#[derive(Debug)]
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

#[derive(PartialEq, Eq)]
enum NumberExtendAction {
    Continue,
    Stop,
}

trait ExtendNumber {
    fn extend(self, number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction;
}

impl ExtendNumber for Uninitialized {
    fn extend(self, number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        let start = buffer.pos();

        let byte = buffer.current_byte().unwrap();

        if byte == b'0' {
            buffer.skip_byte();
            number.end += 1;

            match buffer.byte_at(start + 1) {
                Some(b'x' | b'X') => {
                    buffer.skip_byte();
                    number.end += read::hexadecimal(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'b' | b'B') => {
                    buffer.skip_byte();
                    number.end += read::binary(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'd' | b'D') => {
                    buffer.skip_byte();
                    number.end += read::decimal(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'_') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'o' | b'O') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'0'..=b'7') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer) + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Continue;
                }
                Some(b'8'..=b'9') => {
                    buffer.skip_byte();
                    loop {
                        match buffer.current_byte() {
                            Some(b'_' | b'0'..=b'9') => buffer.skip_byte(),
                            _ => break,
                        }
                    }
                    number.end = buffer.pos();
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Stop;
                }

                _other => {
                    // Sole "0" digit
                    number.kind = NumberKind::Integer(Integer);
                    return NumberExtendAction::Stop;
                }
            }
        }

        // Definitely decimal prefix
        number.end += read::decimal(buffer);
        number.kind = NumberKind::Integer(Integer);
        NumberExtendAction::Continue
    }
}

impl ExtendNumber for Integer {
    fn extend(self, number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        if try_to_extend_with::dot_number_suffix(number, buffer)
            || try_to_extend_with::e_suffix(number, buffer)
            || try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            return NumberExtendAction::Continue;
        }

        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Rational {
    fn extend(self, number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        if try_to_extend_with::i_suffix(number, buffer) {
            return NumberExtendAction::Continue;
        }

        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Imaginary {
    fn extend(self, _number: &mut Number, _buffer: &mut Buffer) -> NumberExtendAction {
        // Imaginary numbers can't be extended to anything bigger
        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Float {
    fn extend(self, number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        if (!self.has_e_suffix && try_to_extend_with::e_suffix(number, buffer))
            || try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            return NumberExtendAction::Continue;
        }

        NumberExtendAction::Stop
    }
}

pub(crate) fn parse_number<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let mut number = Number::new(buffer.pos());
    loop {
        let action = match number.kind {
            NumberKind::Uninitialized(parser) => parser.extend(&mut number, buffer),
            NumberKind::Integer(parser) => parser.extend(&mut number, buffer),
            NumberKind::Rational(parser) => parser.extend(&mut number, buffer),
            NumberKind::Imaginary(parser) => parser.extend(&mut number, buffer),
            NumberKind::Float(parser) => parser.extend(&mut number, buffer),
        };

        if action == NumberExtendAction::Stop {
            break;
        }
    }

    let begin = number.begin;
    let end = number.end;
    let slice = buffer.slice(begin, end);

    let token = match number.kind {
        NumberKind::Uninitialized(_) => unreachable!("ExtendNumber made no transition"),
        NumberKind::Integer(_) => Token(TokenValue::tINTEGER(slice), Loc(begin, end)),
        NumberKind::Rational(_) => Token(TokenValue::tRATIONAL(slice), Loc(begin, end)),
        NumberKind::Imaginary(_) => Token(TokenValue::tIMAGINARY(slice), Loc(begin, end)),
        NumberKind::Float(_) => Token(TokenValue::tFLOAT(slice), Loc(begin, end)),
    };
    println!("{:?}", token);
    token
}

#[cfg(test)]
mod tests;
