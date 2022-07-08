macro_rules! assert_lookahead {
    (test = $test:ident, input = $input:expr, output = $output:expr) => {
        #[test]
        fn $test() {
            #[allow(unused_imports)]
            use crate::lexer::{
                buffer::{Buffer, Lookahead},
                strings::escapes::{SlashU, SlashUError, SlashUPerCodepointError},
            };
            let buffer = Buffer::new($input);
            let lookahead = SlashU::lookahead(&buffer, 0);

            assert_eq!(lookahead, $output);
        }
    };
}

assert_lookahead!(
    test = test_slash_u_nothing,
    input = b"foobar",
    output = Ok(None)
);

// short
assert_lookahead!(
    test = test_slash_u_short_valid,
    input = b"\\u123456",
    output = Ok(Some(SlashU::Short {
        bytes: "\u{1234}".as_bytes().to_vec(),
        length: 6
    }))
);
assert_lookahead!(
    test = test_slash_u_short_invalid,
    input = b"\\uxxxxxx",
    output = Err(SlashUError {
        valid_bytes: None,
        errors: vec![SlashUPerCodepointError::Expected4Got {
            start: 2,
            length: 0
        }],
        length: 2
    })
);

// wide
assert_lookahead!(
    test = test_slash_u_wide_single_codepoint_valid,
    input = b"\\u{1234}",
    output = Ok(Some(SlashU::Wide {
        bytes: "\u{1234}".as_bytes().to_vec(),
        length: 8
    }))
);
assert_lookahead!(
    test = test_slash_u_wide_multiple_codepoint_valid,
    input = b"\\u{ 1234   4321  }",
    output = Ok(Some(SlashU::Wide {
        bytes: "\u{1234}\u{4321}".as_bytes().to_vec(),
        length: 18
    }))
);
assert_lookahead!(
    test = test_slash_u_wide_with_tabs,
    input = b"\\u{ 1234\t\t4321\t}",
    output = Ok(Some(SlashU::Wide {
        bytes: "\u{1234}\u{4321}".as_bytes().to_vec(),
        length: 16 // there are 20 chars - 4 slashes
    }))
);
assert_lookahead!(
    test = test_slash_u_curly_unterminated,
    input = b"\\u{foo123",
    output = Err(SlashUError {
        valid_bytes: None,
        errors: vec![
            SlashUPerCodepointError::NonHex {
                start: 3,
                length: 6
            },
            SlashUPerCodepointError::NoRCurly { start: 9 }
        ],
        length: 9
    })
);
