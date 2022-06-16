use crate::lexer::buffer::{Buffer, Lookahead};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SlashX {
    // Found `\xff`
    Ok { codepoint: u8, length: usize },
    // Found nothing
    Nothing,
    // Found only `\x` but no hex digits
    // In this case length is always `2`
    // but we want it to be explicit
    Err { length: usize },
}

impl<'a> Lookahead<'a> for SlashX {
    type Output = Self;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        if buffer.byte_at(start) != Some(b'\\') && buffer.byte_at(start) != Some(b'x') {
            return SlashX::Nothing;
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
            return SlashX::Err { length: 2 };
        }

        let bytes = buffer
            .slice(codepoint_start, codepoint_start + length)
            .expect("bug");
        let s = std::str::from_utf8(bytes).expect("bug");
        let n = u8::from_str_radix(s, 16).expect("bug");

        SlashX::Ok {
            codepoint: n,
            length: length + 2, // track leading `\x`
        }
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
    output = SlashX::Nothing
);

assert_lookahead!(
    test = test_lookahead_valid_1_digit,
    input = b"\\xF",
    output = SlashX::Ok {
        codepoint: 0xF,
        length: 3
    }
);

assert_lookahead!(
    test = test_lookahead_valid_2_digits,
    input = b"\\xFF",
    output = SlashX::Ok {
        codepoint: 0xFF,
        length: 4
    }
);

assert_lookahead!(
    test = test_lookahead_invalid_non_hex,
    input = b"\\xI",
    output = SlashX::Err { length: 2 }
);

assert_lookahead!(
    test = test_lookahead_invalid_eof,
    input = b"\\x",
    output = SlashX::Err { length: 2 }
);
