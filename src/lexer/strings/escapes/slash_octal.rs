use crate::lexer::buffer::{Buffer, Lookahead};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashOctal {
    // Found `\XXX`
    codepoint: u8,
    length: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct SlashOctalError {
    // Found only `\`, but no octal number(s)
    // In this case length is always `1`
    // but we want it to be explicit
    length: u8,
}

impl<'a> Lookahead<'a> for SlashOctal {
    type Output = Result<Option<Self>, SlashOctalError>;

    fn lookahead(buffer: &Buffer<'a>, start: usize) -> Self::Output {
        if buffer.byte_at(start) != Some(b'\\') {
            return Ok(None);
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
            return Err(SlashOctalError { length: 1 });
        }

        let bytes = buffer
            .slice(codepoint_start, codepoint_start + length)
            .expect("bug");
        let s = std::str::from_utf8(bytes).expect("bug");
        let n = u16::from_str_radix(s, 8).expect("bug");
        let n = (n % 256) as u8;

        Ok(Some(SlashOctal {
            codepoint: n,
            length: length + 1, // track leading `\`
        }))
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

assert_lookahead!(
    test = test_lookahead_nothing,
    input = b"foo",
    output = Ok(None)
);

assert_lookahead!(
    test = test_lookahead_valid_3_digits,
    input = b"\\111",
    output = Ok(Some(SlashOctal {
        codepoint: 73,
        length: 4
    }))
);
assert_lookahead!(
    test = test_lookahead_valid_2_digits,
    input = b"\\33",
    output = Ok(Some(SlashOctal {
        codepoint: 27,
        length: 3
    }))
);
assert_lookahead!(
    test = test_lookahead_valid_1_digit,
    input = b"\\7",
    output = Ok(Some(SlashOctal {
        codepoint: 7,
        length: 2
    }))
);

assert_lookahead!(
    test = test_lookahead_invalid_nonoctal,
    input = b"\\9",
    output = Err(SlashOctalError { length: 1 })
);

assert_lookahead!(
    test = test_lookahead_invalid_eof,
    input = b"\\",
    output = Err(SlashOctalError { length: 1 })
);
