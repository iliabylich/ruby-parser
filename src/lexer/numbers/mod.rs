use std::ops::ControlFlow;

use crate::lexer::buffer::Buffer;
use crate::token::{token, Token};

pub(crate) mod read;
pub(crate) mod try_to_extend_with;

#[derive(Clone, Copy, Debug)]
struct Uninitialized;
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
    Integer(Integer),
    Rational(Rational),
    Imaginary(Imaginary),
    FloatWithDotNumber(FloatWithDotNumber),
    FloatWithESuffix(FloatWithESuffix),
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

trait ExtendNumber {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()>;
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
                    number.end += read::hexadecimal(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
                }
                Some(b'b' | b'B') => {
                    buffer.skip_byte();
                    number.end += read::binary(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
                }
                Some(b'd' | b'D') => {
                    buffer.skip_byte();
                    number.end += read::decimal(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
                }
                Some(b'_') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
                }
                Some(b'o' | b'O') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
                }
                Some(b'0'..=b'7') => {
                    buffer.skip_byte();
                    number.end += read::octal(buffer)
                        .expect("numeric literal without digits")
                        .get()
                        + 1;
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Continue(());
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
                    return ControlFlow::Break(());
                }

                _other => {
                    // Sole "0" digit
                    number.kind = NumberKind::Integer(Integer);
                    return ControlFlow::Break(());
                }
            }
        }

        // Definitely decimal prefix
        number.end += read::decimal(buffer)
            .expect("numeric literal without digits")
            .get();
        number.kind = NumberKind::Integer(Integer);
        ControlFlow::Continue(())
    }
}

impl ExtendNumber for Integer {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        if try_to_extend_with::dot_number_suffix(number, buffer)
            || try_to_extend_with::e_suffix(number, buffer)
            || try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl ExtendNumber for Rational {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        if try_to_extend_with::i_suffix(number, buffer) {
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl ExtendNumber for Imaginary {
    fn extend(_number: &mut Number, _buffer: &mut Buffer) -> ControlFlow<()> {
        // Imaginary numbers can't be extended to anything bigger
        ControlFlow::Break(())
    }
}

impl ExtendNumber for FloatWithDotNumber {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        if try_to_extend_with::e_suffix(number, buffer)
            || try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            ControlFlow::Continue(())
        } else {
            ControlFlow::Break(())
        }
    }
}

impl ExtendNumber for FloatWithESuffix {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        if try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            return ControlFlow::Continue(());
        }

        ControlFlow::Break(())
    }
}

impl ExtendNumber for Number {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> ControlFlow<()> {
        match number.kind {
            NumberKind::Uninitialized(_) => Uninitialized::extend(number, buffer),
            NumberKind::Integer(_) => Integer::extend(number, buffer),
            NumberKind::Rational(_) => Rational::extend(number, buffer),
            NumberKind::Imaginary(_) => Imaginary::extend(number, buffer),
            NumberKind::FloatWithDotNumber(_) => FloatWithDotNumber::extend(number, buffer),
            NumberKind::FloatWithESuffix(_) => FloatWithESuffix::extend(number, buffer),
        }
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

    let begin = number.begin;
    let end = number.end;

    let token = match number.kind {
        NumberKind::Uninitialized(_) => unreachable!("ExtendNumber made no transition"),
        NumberKind::Integer(_) => token!(tINTEGER, begin, end),
        NumberKind::Rational(_) => token!(tRATIONAL, begin, end),
        NumberKind::Imaginary(_) => token!(tIMAGINARY, begin, end),
        NumberKind::FloatWithDotNumber(_) => token!(tFLOAT, begin, end),
        NumberKind::FloatWithESuffix(_) => token!(tFLOAT, begin, end),
    };
    println!("{:?}", token);
    token
}

#[cfg(test)]
mod tests;
