use crate::lexer::numbers::{read, Buffer, Float, Imaginary, Number, NumberKind, Rational};

pub(crate) fn dot_number_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    // Do not let it to be parsed twice
    debug_assert!(!matches!(
        number.kind,
        NumberKind::Float(Float {
            has_dot_number_suffix: true,
            ..
        })
    ));

    let start = buffer.pos();

    if buffer.byte_at(start) != Some(b'.') {
        return false;
    }
    buffer.skip_byte();

    match read::decimal(buffer) {
        0 => {
            // rollback
            buffer.set_pos(start);
            false
        }
        mut len => {
            // track leading '.'
            len += 1;
            buffer.set_pos(start + len);
            // extend to float
            number.end += len;
            number.kind = NumberKind::Float(Float {
                has_dot_number_suffix: true,
                has_e_suffix: false,
            });
            true
        }
    }
}

pub(crate) fn e_suffix(number: &mut Number, buffer: &mut Buffer) -> bool {
    // Do not let it to be parsed twice
    debug_assert!(!matches!(
        number.kind,
        NumberKind::Float(Float {
            has_e_suffix: true,
            ..
        })
    ));

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
        0 => {
            // rollback
            buffer.set_pos(start);
            false
        }
        mut len => {
            // track leading sign and 'e'
            len += 1 + sign_length;
            buffer.set_pos(start + len);
            // extend to float
            number.end += len;
            number.kind = NumberKind::Float(Float {
                has_dot_number_suffix: false,
                has_e_suffix: true,
            });
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
