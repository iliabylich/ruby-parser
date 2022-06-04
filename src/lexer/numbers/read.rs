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

pub(crate) fn hexadecimal(buffer: &mut Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    grab_integer_with_numbers!(buffer, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F');
    std::num::NonZeroUsize::new(buffer.pos() - start)
}

pub(crate) fn binary(buffer: &mut Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    grab_integer_with_numbers!(buffer, b'0' | b'1');
    std::num::NonZeroUsize::new(buffer.pos() - start)
}

pub(crate) fn decimal(buffer: &mut Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    grab_integer_with_numbers!(buffer, b'0'..=b'9');
    std::num::NonZeroUsize::new(buffer.pos() - start)
}

pub(crate) fn octal(buffer: &mut Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    grab_integer_with_numbers!(buffer, b'0'..=b'7');
    std::num::NonZeroUsize::new(buffer.pos() - start)
}
