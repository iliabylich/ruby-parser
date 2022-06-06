use crate::lexer::numbers::{
    read, Buffer, FloatWithDotNumber, FloatWithESuffix, Imaginary, Number, NumberKind, Rational,
};

pub(crate) fn dot_number_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    let start = buffer.pos();

    if buffer.byte_at(start) != Some(b'.') {
        return false;
    }
    buffer.skip_byte();

    match read::decimal(buffer) {
        None => {
            // rollback
            buffer.set_pos(start);
            false
        }
        Some(len) => {
            // track leading '.'
            let len = len.get() + 1;
            buffer.set_pos(start + len);
            // extend to float
            number.end += len;
            number.kind = NumberKind::FloatWithDotNumber(FloatWithDotNumber);
            true
        }
    }
}

pub(crate) fn e_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    let start = buffer.pos();

    if !matches!(buffer.byte_at(start), Some(b'e' | b'E')) {
        return false;
    }
    buffer.skip_byte();

    // consume optional sign
    let mut sign_length = 0;
    if matches!(buffer.byte_at(start + 1), Some(b'-' | b'+')) {
        sign_length = 1;
        buffer.skip_byte();
    }

    match read::decimal(buffer) {
        None => {
            // rollback
            buffer.set_pos(start);
            false
        }
        Some(len) => {
            // track leading sign and 'e'
            let len = len.get() + 1 + sign_length;
            buffer.set_pos(start + len);
            // extend to float
            number.end += len;
            number.kind = NumberKind::FloatWithESuffix(FloatWithESuffix);
            true
        }
    }
}

pub(crate) fn r_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    if buffer.current_byte() != Some(b'r') {
        return false;
    }
    // TODO: check lookahead (like 'rescue')
    buffer.skip_byte();
    number.end += 1;
    number.kind = NumberKind::Rational(Rational);
    return true;
}

pub(crate) fn i_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    if buffer.current_byte() != Some(b'i') {
        return false;
    }
    // TODO: check lookahead (like 'if')
    buffer.skip_byte();
    number.end += 1;
    number.kind = NumberKind::Imaginary(Imaginary);
    return true;
}
