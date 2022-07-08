use crate::lexer::buffer::{Buffer, Lookahead};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashOctal {
    // Found `\XXX`
    pub(crate) byte: u8,
    pub(crate) length: usize,
}

impl Lookahead for SlashOctal {
    type Output = Option<Self>;

    fn lookahead(buffer: &Buffer, start: usize) -> Self::Output {
        if !buffer.lookahead(start, b"\\") {
            return None;
        }

        let codepoint_start = start + 1;
        let mut length = 0;

        for i in 0..3 {
            match buffer.byte_at(codepoint_start + i) {
                Some(b'0'..=b'7') => length = i + 1,
                _ => break,
            }
        }

        if length == 0 {
            // just a `\`, possible a different escape sequence
            return None;
        }

        let bytes = buffer
            .slice(codepoint_start, codepoint_start + length)
            .expect("bug");
        let s = std::str::from_utf8(bytes).expect("bug");
        let n = u16::from_str_radix(s, 8).expect("bug");
        let n = (n % 256) as u8;

        Some(SlashOctal {
            byte: n,
            length: length + 1, // track leading `\`
        })
    }
}

macro_rules! assert_lookahead {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            let buffer = crate::lexer::buffer::Buffer::new($input);
            let lookahead = SlashOctal::lookahead(&buffer, 0);
            assert_eq!(lookahead, $output);
        }
    };
}

assert_lookahead!(test = test_lookahead_nothing, input = b"foo", output = None);

assert_lookahead!(
    test = test_lookahead_valid_3_digits,
    input = b"\\111",
    output = Some(SlashOctal {
        byte: 73,
        length: 4
    })
);
assert_lookahead!(
    test = test_lookahead_valid_2_digits,
    input = b"\\33",
    output = Some(SlashOctal {
        byte: 27,
        length: 3
    })
);
assert_lookahead!(
    test = test_lookahead_valid_1_digit,
    input = b"\\7",
    output = Some(SlashOctal { byte: 7, length: 2 })
);

assert_lookahead!(
    test = test_lookahead_invalid_nonoctal,
    input = b"\\9",
    output = None
);

assert_lookahead!(
    test = test_lookahead_invalid_eof,
    input = b"\\",
    output = None
);
