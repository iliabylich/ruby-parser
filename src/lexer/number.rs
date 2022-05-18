use crate::lexer::{assert_lex, buffer::Buffer};
use crate::token::{Loc, Token, TokenValue};

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
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction;
}

impl ExtendNumber for Uninitialized {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        let start = buffer.pos();

        let byte = buffer.current_byte()
            .expect("bug: ExtendNumber for Uninitialized state can be called only at the beginning of a number");

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

mod read {
    use super::Buffer;

    macro_rules! grab_integer_with_numbers {
        ($buffer:expr, $pat:pat) => {
            loop {
                match $buffer.current_byte() {
                    Some($pat) => $buffer.skip_byte(),
                    Some(b'_') => {
                        if $buffer.byte_at($buffer.pos() - 1) == Some(b'_') {
                            // reject 2 cons '_' bytes
                            break;
                        } else {
                            $buffer.skip_byte();
                        }
                    }
                    _other => break,
                }
            }
            // Discard trailing '_'
            if $buffer.byte_at($buffer.pos() - 1) == Some(b'_') {
                $buffer.set_pos($buffer.pos() - 1);
            }
        };
    }

    pub(crate) fn hexadecimal(buffer: &mut Buffer) -> usize {
        let start = buffer.pos();
        grab_integer_with_numbers!(buffer, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F');
        buffer.pos() - start
    }

    pub(crate) fn binary(buffer: &mut Buffer) -> usize {
        let start = buffer.pos();
        grab_integer_with_numbers!(buffer, b'0' | b'1');
        buffer.pos() - start
    }

    pub(crate) fn decimal(buffer: &mut Buffer) -> usize {
        let start = buffer.pos();
        grab_integer_with_numbers!(buffer, b'0'..=b'9');
        buffer.pos() - start
    }

    pub(crate) fn octal(buffer: &mut Buffer) -> usize {
        let start = buffer.pos();
        grab_integer_with_numbers!(buffer, b'0'..=b'7');
        buffer.pos() - start
    }
}

mod try_to_extend_with {
    use super::{read, Buffer, Float, Imaginary, Number, NumberKind, Rational};

    pub(crate) fn dot_number_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
        // Do not let it to be parsed twice
        match number.kind {
            NumberKind::Float(Float {
                has_dot_number_suffix,
                ..
            }) if has_dot_number_suffix => return false,
            _ => {}
        }

        let start = buffer.pos();

        let dot_number_float_suffix_len = {
            if buffer.byte_at(start) == Some(b'.') {
                buffer.skip_byte();
                let mut suffix_len = read::decimal(buffer);
                if suffix_len == 0 {
                    // rollback
                    buffer.set_pos(start);
                    0
                } else {
                    // track leading '.'
                    suffix_len += 1;
                    buffer.set_pos(start + suffix_len);
                    suffix_len
                }
            } else {
                // No ".ddd" suffix
                0
            }
        };

        if dot_number_float_suffix_len > 0 {
            // extend to float
            number.end += dot_number_float_suffix_len;
            number.kind = NumberKind::Float(Float {
                has_dot_number_suffix: true,
                has_e_suffix: false,
            });
            return true;
        }
        false
    }

    pub(crate) fn e_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
        // Do not let it to be parsed twice
        match number.kind {
            NumberKind::Float(Float { has_e_suffix, .. }) if has_e_suffix => return false,
            _ => {}
        }

        let e_float_suffix_len = {
            let start = buffer.pos();

            if matches!(buffer.byte_at(start), Some(b'e' | b'E')) {
                buffer.skip_byte();

                let mut sign_length = 0;
                if matches!(buffer.byte_at(start + 1), Some(b'-' | b'+')) {
                    sign_length = 1;
                    buffer.skip_byte();
                }

                let mut suffix_len = read::decimal(buffer);
                if suffix_len == 0 {
                    // rollback
                    buffer.set_pos(0);
                    0
                } else {
                    // track leading 'e' and sign
                    suffix_len += 1 + sign_length;
                    buffer.set_pos(start + suffix_len);
                    suffix_len
                }
            } else {
                // No 'e' suffix
                0
            }
        };

        if e_float_suffix_len > 0 {
            // extend to float
            number.end += e_float_suffix_len;
            number.kind = NumberKind::Float(Float {
                has_dot_number_suffix: false,
                has_e_suffix: true,
            });
            return true;
        }
        false
    }

    pub(crate) fn r_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
        if buffer.current_byte() == Some(b'r') {
            // TODO: check lookahead (like 'rescue')
            buffer.skip_byte();
            number.end += 1;
            number.kind = NumberKind::Rational(Rational);
            return true;
        }
        false
    }

    pub(crate) fn i_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
        if buffer.current_byte() == Some(b'i') {
            // TODO: check lookahead (like 'if')
            buffer.skip_byte();
            number.end += 1;
            number.kind = NumberKind::Imaginary(Imaginary);
            return true;
        }
        false
    }
}

impl ExtendNumber for Integer {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
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
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        if try_to_extend_with::i_suffix(number, buffer) {
            return NumberExtendAction::Continue;
        }

        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Imaginary {
    fn extend(_number: &mut Number, _buffer: &mut Buffer) -> NumberExtendAction {
        // Imaginary numbers can't be extended to anything bigger
        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Float {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        if try_to_extend_with::e_suffix(number, buffer)
            || try_to_extend_with::r_suffix(number, buffer)
            || try_to_extend_with::i_suffix(number, buffer)
        {
            return NumberExtendAction::Continue;
        }

        NumberExtendAction::Stop
    }
}

impl ExtendNumber for Number {
    fn extend(number: &mut Number, buffer: &mut Buffer) -> NumberExtendAction {
        match number.kind {
            NumberKind::Uninitialized(_) => Uninitialized::extend(number, buffer),
            NumberKind::Integer(_) => Integer::extend(number, buffer),
            NumberKind::Rational(_) => Rational::extend(number, buffer),
            NumberKind::Imaginary(_) => Imaginary::extend(number, buffer),
            NumberKind::Float(_) => Float::extend(number, buffer),
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
        NumberKind::Uninitialized(_) => unreachable!("ExtendNumber made no transition"),
        NumberKind::Integer(_) => Token(TokenValue::tINTEGER(slice), Loc(begin, end)),
        NumberKind::Rational(_) => Token(TokenValue::tRATIONAL(slice), Loc(begin, end)),
        NumberKind::Imaginary(_) => Token(TokenValue::tIMAGINARY(slice), Loc(begin, end)),
        NumberKind::Float(_) => Token(TokenValue::tFLOAT(slice), Loc(begin, end)),
    };
    println!("{:?}", token);
    token
}

mod prefix_tests {
    use super::*;

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
}

mod basic_decimal_tests {
    use super::*;

    assert_lex!(test_tINTEGER_decimal, "42", tINTEGER(b"42"), 0..2);
}

mod underscore_tests {
    use super::*;

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
}

mod trailing_underscore_tests {
    use super::*;

    assert_lex!(
        test_tINTEGER_hexadecimal_with_trailing_underscore,
        "0x1_",
        tINTEGER(b"0x1"),
        0..3
    );

    assert_lex!(
        test_tINTEGER_binary_with_trailing_underscore,
        "0b1_",
        tINTEGER(b"0b1"),
        0..3
    );

    assert_lex!(
        test_tINTEGER_decimal_with_trailing_underscore,
        "0d8_",
        tINTEGER(b"0d8"),
        0..3
    );

    assert_lex!(
        test_tINTEGER_octal_with_trailing_underscore,
        "02_",
        tINTEGER(b"02"),
        0..2
    );
}

mod float_tests {
    use super::*;
    assert_lex!(test_tFLOAT_plain, "12.34", tFLOAT(b"12.34"), 0..5);

    assert_lex!(test_tFLOAT_int_lower_e, "1e3", tFLOAT(b"1e3"), 0..3);
    assert_lex!(test_tFLOAT_int_plus_lower_e, "1e+3", tFLOAT(b"1e+3"), 0..4);
    assert_lex!(test_tFLOAT_int_minus_lower_e, "1e-3", tFLOAT(b"1e-3"), 0..4);
    assert_lex!(test_tFLOAT_float_lower_e, "1.2e3", tFLOAT(b"1.2e3"), 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_lower_e,
        "1.2e+3",
        tFLOAT(b"1.2e+3"),
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_lower_e,
        "1.2e-3",
        tFLOAT(b"1.2e-3"),
        0..6
    );

    assert_lex!(test_tFLOAT_int_upper_e, "1E3", tFLOAT(b"1E3"), 0..3);
    assert_lex!(test_tFLOAT_int_plus_upper_e, "1E+3", tFLOAT(b"1E+3"), 0..4);
    assert_lex!(test_tFLOAT_int_minus_upper_e, "1E-3", tFLOAT(b"1E-3"), 0..4);
    assert_lex!(test_tFLOAT_float_upper_e, "1.2E3", tFLOAT(b"1.2E3"), 0..5);
    assert_lex!(
        test_tFLOAT_float_plus_upper_e,
        "1.2E+3",
        tFLOAT(b"1.2E+3"),
        0..6
    );
    assert_lex!(
        test_tFLOAT_float_minus_upper_e,
        "1.2E-3",
        tFLOAT(b"1.2E-3"),
        0..6
    );
}

mod rational_tests {
    use super::*;

    assert_lex!(test_tRATIONAL_int, "1r", tRATIONAL(b"1r"), 0..2);
    assert_lex!(test_tRATIONAL_float, "1.2r", tRATIONAL(b"1.2r"), 0..4);
}

mod imaginary_tests {
    use super::*;

    assert_lex!(test_tIMAGINARY_int, "1i", tIMAGINARY(b"1i"), 0..2);
    assert_lex!(test_tIMAGINARY_float, "1.2i", tIMAGINARY(b"1.2i"), 0..4);
}

mod rational_and_imaginary_tests {
    use super::*;

    assert_lex!(
        test_tIMAGINARY_rational_int,
        "1ri",
        tIMAGINARY(b"1ri"),
        0..3
    );
    assert_lex!(
        test_tIMAGINARY_rational_float,
        "1.2ri",
        tIMAGINARY(b"1.2ri"),
        0..5
    );
}
