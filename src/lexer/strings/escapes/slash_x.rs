use crate::lexer::buffer::{Buffer, Lookahead};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashX {
    // Found `\xff`
    pub(crate) codepoint: u8,
    pub(crate) length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashXError {
    // Found only `\x` but no hex digits
    // In this case length is always `2`
    // but we want it to be explicit
    pub(crate) length: usize,
}

impl<'a> Lookahead<'a> for SlashX {
    type Output = Result<Option<Self>, SlashXError>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        if !buffer.lookahead(start, b"\\x") {
            return Ok(None);
        }

        let codepoint_start = start + 2;
        let mut length = 0;

        for i in 0..2 {
            match buffer.byte_at(codepoint_start + i) {
                Some(b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F') => length = i + 1,
                _ => break,
            }
        }

        if length == 0 {
            return Err(SlashXError { length: 2 });
        }

        let bytes = buffer
            .slice(codepoint_start, codepoint_start + length)
            .expect("bug");
        let s = std::str::from_utf8(bytes).expect("bug");
        let n = u8::from_str_radix(s, 16).expect("bug");

        Ok(Some(SlashX {
            codepoint: n,
            length: length + 2, // track leading `\x`
        }))
    }
}

macro_rules! assert_lookahead {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            let buffer = crate::lexer::buffer::Buffer::new($input);
            let lookahead = SlashX::lookahead(&buffer, 0);
            assert_eq!(lookahead, $output);
        }
    };
}

assert_lookahead!(
    test = test_lookahead_nothing,
    input = b"foobar",
    output = Ok(None)
);

assert_lookahead!(
    test = test_lookahead_valid_1_digit,
    input = b"\\xF",
    output = Ok(Some(SlashX {
        codepoint: 0xF,
        length: 3
    }))
);

assert_lookahead!(
    test = test_lookahead_valid_2_digits,
    input = b"\\xFF",
    output = Ok(Some(SlashX {
        codepoint: 0xFF,
        length: 4
    }))
);

assert_lookahead!(
    test = test_lookahead_invalid_non_hex,
    input = b"\\xI",
    output = Err(SlashXError { length: 2 })
);

assert_lookahead!(
    test = test_lookahead_invalid_eof,
    input = b"\\x",
    output = Err(SlashXError { length: 2 })
);
