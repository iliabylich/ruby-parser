use crate::lexer::{buffer::Buffer, strings::escapes::unescape_byte};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashByte {
    pub(crate) byte: u8,
    pub(crate) length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashByteError {
    pub(crate) length: usize,
}

impl SlashByte {
    pub(crate) fn lookahead(buffer: &Buffer, start: usize) -> Result<Option<Self>, SlashByteError> {
        if buffer.byte_at(start) != Some(b'\\') {
            return Ok(None);
        }

        if let Some(byte) = buffer.byte_at(start + 1) {
            let byte = unescape_byte(byte);
            Ok(Some(SlashByte { byte, length: 2 }))
        } else {
            Err(SlashByteError { length: 1 })
        }
    }
}

macro_rules! assert_lookahead {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            // use crate::lexer::{buffer::{Buffer, Lookahead}};
            let buffer = Buffer::new($input);
            let lookahead = SlashByte::lookahead(&buffer, 0);
            assert_eq!(lookahead, $output);
        }
    };
}

assert_lookahead!(test = test_nothing, input = b"foobar", output = Ok(None));

assert_lookahead!(
    test = test_ok_special_byte,
    input = b"\\\n",
    output = Ok(Some(SlashByte {
        byte: b'\n',
        length: 2
    }))
);

assert_lookahead!(
    test = test_ok_normal_byte,
    input = b"\\d",
    output = Ok(Some(SlashByte {
        byte: b'd',
        length: 2
    }))
);

assert_lookahead!(
    test = test_eof,
    input = b"\\",
    output = Err(SlashByteError { length: 1 })
);
