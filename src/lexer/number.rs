use crate::lexer::{assert_lex, buffer::Buffer};
use crate::token::{Loc, Token, TokenValue};

#[derive(Clone, Copy)]
struct Uninitialized;
#[derive(Clone, Copy)]
struct Integer;
#[derive(Clone, Copy)]
struct Rational;
#[derive(Clone, Copy)]
struct Complex;
#[derive(Clone, Copy)]
struct Float;

#[derive(Clone, Copy)]
enum NumberKind {
    Uninitialized,
    Integer,
    Rational,
    Complex,
    Float,
}

struct Number {
    kind: NumberKind,
    begin: usize,
    end: usize,
}

impl Number {
    fn new(start: usize) -> Self {
        Self {
            kind: NumberKind::Uninitialized,
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
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction;
}

impl ExtendNumber for Uninitialized {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        let start = buffer.pos();

        // SAFETY: we call parse_number after seeing a digit
        //         and jumping back to pre-number position,
        //         so .current_byte() always returns a digit here.
        let byte = buffer.current_byte().expect("bug: ExtendNumber for Uninitialized state can be called only at the beginning of a number");

        if byte == b'0' {
            buffer.skip_byte();
            number.end += 1;

            match buffer.byte_at(start + 1) {
                Some(b'x' | b'X') => {
                    buffer.skip_byte();
                    number.end += read_hexadecimal(buffer) + 1;
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Continue;
                }
                Some(b'b' | b'B') => {
                    buffer.skip_byte();
                    number.end += read_binary(buffer) + 1;
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Continue;
                }
                Some(b'd' | b'D') => {
                    buffer.skip_byte();
                    number.end += read_decimal(buffer) + 1;
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Continue;
                }
                Some(b'_') => {
                    buffer.skip_byte();
                    number.end += read_octal(buffer) + 1;
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Continue;
                }
                Some(b'o' | b'O') => {
                    buffer.skip_byte();
                    number.end += read_octal(buffer) + 1;
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Continue;
                }
                Some(b'0'..=b'7') => {
                    buffer.skip_byte();
                    number.end += read_octal(buffer) + 1;
                    number.kind = NumberKind::Integer;
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
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Stop;
                }

                _other => {
                    // Sole "0" digit
                    number.kind = NumberKind::Integer;
                    return NumberExtendAction::Stop;
                }
            }
        }

        // Definitely decimal prefix
        number.end += read_decimal(buffer);
        number.kind = NumberKind::Integer;
        NumberExtendAction::Continue
    }
}

impl ExtendNumber for Integer {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        // todo!("ExtendNumber for Integer")
        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Rational {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        todo!("ExtendNumber for Rational")
    }
}

impl ExtendNumber for Complex {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        todo!("ExtendNumber for Complex")
    }
}

impl ExtendNumber for Float {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        todo!("ExtendNumber for Float")
    }
}

impl ExtendNumber for Number {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        match number.kind {
            NumberKind::Uninitialized => Uninitialized::extend(number, buffer),
            NumberKind::Integer => Integer::extend(number, buffer),
            NumberKind::Rational => Rational::extend(number, buffer),
            NumberKind::Complex => Complex::extend(number, buffer),
            NumberKind::Float => Float::extend(number, buffer),
        }
    }
}

pub(crate) fn parse_number<'a>(buffer: &mut Buffer<'a>) -> Token<'a> {
    let mut number = Number::new(buffer.pos());
    while Number::extend(&mut number, buffer) == NumberExtendAction::Continue {}

    let begin = number.begin;
    let end = number.end;
    let slice = buffer.slice(begin, end);

    let token = match number.kind {
        NumberKind::Uninitialized => unreachable!("ExtendNumber made no transition"),
        NumberKind::Integer => Token(TokenValue::tINTEGER(slice), Loc(begin, end)),
        NumberKind::Rational => Token(TokenValue::tRATIONAL(slice), Loc(begin, end)),
        NumberKind::Complex => Token(TokenValue::tIMAGINARY(slice), Loc(begin, end)),
        NumberKind::Float => todo!(),
    };
    println!("{:?}", token);
    token
}

fn read_hexadecimal<'a>(buffer: &mut Buffer<'a>) -> usize {
    let start = buffer.pos();
    while matches!(
        buffer.current_byte(),
        Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' | b'_')
    ) {
        buffer.skip_byte();
    }
    buffer.pos() - start
}

fn read_binary<'a>(buffer: &mut Buffer<'a>) -> usize {
    let start = buffer.pos();
    while matches!(buffer.current_byte(), Some(b'0' | b'1' | b'_')) {
        buffer.skip_byte();
    }
    buffer.pos() - start
}

fn read_decimal<'a>(buffer: &mut Buffer<'a>) -> usize {
    let start = buffer.pos();
    while matches!(buffer.current_byte(), Some(b'0'..=b'9' | b'_')) {
        buffer.skip_byte();
    }
    buffer.pos() - start
}

fn read_octal<'a>(buffer: &mut Buffer<'a>) -> usize {
    let start = buffer.pos();
    while matches!(buffer.current_byte(), Some(b'0'..=b'7' | b'_')) {
        buffer.skip_byte();
    }
    buffer.pos() - start
}

// Test prefixes
assert_lex!(
    test_tINTEGER_hexadecimal_prefix_lower,
    "0x42",
    tINTEGER(b"0x42"),
    0..4
);
assert_lex!(
    test_tINTEGER_hexadecimal_prefix_upper,
    "0X42",
    tINTEGER(b"0X42"),
    0..4
);
assert_lex!(
    test_tINTEGER_binary_prefix_lower,
    "0b1010",
    tINTEGER(b"0b1010"),
    0..6
);
assert_lex!(
    test_tINTEGER_binary_prefix_upper,
    "0B1010",
    tINTEGER(b"0B1010"),
    0..6
);
assert_lex!(
    test_tINTEGER_decimal_prefix_lower,
    "0d192",
    tINTEGER(b"0d192"),
    0..5
);
assert_lex!(
    test_tINTEGER_decimal_prefix_upper,
    "0D192",
    tINTEGER(b"0D192"),
    0..5
);
assert_lex!(
    test_tINTEGER_octal_prefix_underscore,
    "0_12",
    tINTEGER(b"0_12"),
    0..4
);
assert_lex!(
    test_tINTEGER_octal_prefix_lower,
    "0o12",
    tINTEGER(b"0o12"),
    0..4
);
assert_lex!(
    test_tINTEGER_octal_prefix_upper,
    "0O12",
    tINTEGER(b"0O12"),
    0..4
);

// Test basic decimal
assert_lex!(test_tINTEGER_decimal, "42", tINTEGER(b"42"), 0..2);

// Test undescores
assert_lex!(
    test_tINTEGER_hexadecimal_with_underscores,
    "0x1_2",
    tINTEGER(b"0x1_2"),
    0..5
);

assert_lex!(
    test_tINTEGER_binary_with_underscores,
    "0b1_0",
    tINTEGER(b"0b1_0"),
    0..5
);

assert_lex!(
    test_tINTEGER_decimal_with_underscores,
    "0d8_9",
    tINTEGER(b"0d8_9"),
    0..5
);

assert_lex!(
    test_tINTEGER_octal_with_underscores,
    "02_7",
    tINTEGER(b"02_7"),
    0..4
);
