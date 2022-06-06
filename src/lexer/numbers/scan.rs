use crate::lexer::buffer::Buffer;

macro_rules! read_while_digits {
    ($buffer:expr, $start:ident, $pat:pat) => {{
        let mut end = $start;
        loop {
            match $buffer.byte_at(end) {
                Some($pat) => {
                    end += 1;
                }
                Some(b'_') => {
                    if $buffer.byte_at(end - 1) == Some(b'_') {
                        // reject 2 cons '_' bytes
                        break;
                    } else {
                        end += 1;
                    }
                }
                _other => break,
            }
        }
        // Discard trailing '_'
        if $buffer.byte_at(end - 1) == Some(b'_') {
            end -= 1
        }

        end
    }};
}

pub(crate) fn hexadecimal(buffer: &Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    let end = read_while_digits!(buffer, start, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F');
    std::num::NonZeroUsize::new(end - start)
}

pub(crate) fn binary(buffer: &Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    let end = read_while_digits!(buffer, start, b'0' | b'1');
    std::num::NonZeroUsize::new(end - start)
}

pub(crate) fn decimal(buffer: &Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    let end = read_while_digits!(buffer, start, b'0'..=b'9');
    std::num::NonZeroUsize::new(end - start)
}

pub(crate) fn octal(buffer: &Buffer) -> Option<std::num::NonZeroUsize> {
    let start = buffer.pos();
    let end = read_while_digits!(buffer, start, b'0'..=b'7');
    std::num::NonZeroUsize::new(end - start)
}
