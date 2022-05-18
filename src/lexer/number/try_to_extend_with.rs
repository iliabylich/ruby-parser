use crate::lexer::number::{read, Buffer, Float, Imaginary, Number, NumberKind, Rational};

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
    debug_assert!(!matches!(
        number.kind,
        NumberKind::Float(Float {
            has_e_suffix: true,
            ..
        })
    ));

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
