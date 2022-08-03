macro_rules! assert_lookahead {
    (
        test = $test:ident,
        input = $input:expr,
        output = $output:expr,
        unescaped = $unescaped:expr
    ) => {
        #[test]
        fn $test() {
            #[allow(unused_imports)]
            use crate::{
                buffer::Buffer,
                lexer::strings::escapes::{SlashU, SlashUError, SlashUPerCodepointError},
                loc::Loc,
            };
            let mut buffer = Buffer::new($input);
            let lookahead = SlashU::lookahead(&mut buffer, 0);

            assert_eq!(lookahead, $output);

            let unescaped: Option<&[u8]> = match lookahead {
                Ok(Some(SlashU::Wide { escaped_loc, .. })) => {
                    let slice = buffer
                        .unescaped_slice_at(escaped_loc.start, escaped_loc.end)
                        .unwrap();
                    Some(slice)
                }
                _ => None,
            };
            assert_eq!(unescaped, $unescaped);
        }
    };
}

assert_lookahead!(
    test = test_slash_u_nothing,
    input = b"foobar",
    output = Ok(None),
    unescaped = None
);

// short
assert_lookahead!(
    test = test_slash_u_short_valid,
    input = b"\\u123456",
    output = Ok(Some(SlashU::Short {
        codepoint: '\u{1234}',
        length: 6
    })),
    unescaped = None
);
assert_lookahead!(
    test = test_slash_u_short_invalid,
    input = b"\\uxxxxxx",
    output = Err(SlashUError {
        escaped_loc: Loc { start: 0, end: 0 },
        errors: vec![SlashUPerCodepointError::Expected4Got {
            start: 2,
            length: 0
        }],
        length: 2
    }),
    unescaped = None
);

// wide
assert_lookahead!(
    test = test_slash_u_wide_single_codepoint_valid,
    input = b"\\u{1234}",
    output = Ok(Some(SlashU::Wide {
        escaped_loc: Loc { start: 0, end: 3 },
        length: 8
    })),
    unescaped = Some("\u{1234}".as_bytes())
);
assert_lookahead!(
    test = test_slash_u_wide_multiple_codepoint_valid,
    input = b"\\u{ 1234   4321  }",
    output = Ok(Some(SlashU::Wide {
        escaped_loc: Loc { start: 0, end: 6 },
        length: 18
    })),
    unescaped = Some("\u{1234}\u{4321}".as_bytes())
);
assert_lookahead!(
    test = test_slash_u_wide_with_tabs,
    input = b"\\u{ 1234\t\t4321\t}",
    output = Ok(Some(SlashU::Wide {
        escaped_loc: Loc { start: 0, end: 6 },
        length: 16 // there are 20 chars - 4 slashes
    })),
    unescaped = Some("\u{1234}\u{4321}".as_bytes())
);
assert_lookahead!(
    test = test_slash_u_curly_unterminated,
    input = b"\\u{foo123",
    output = Err(SlashUError {
        escaped_loc: Loc { start: 0, end: 0 },
        errors: vec![
            SlashUPerCodepointError::NonHex {
                start: 3,
                length: 6
            },
            SlashUPerCodepointError::NoRCurly { start: 9 }
        ],
        length: 9
    }),
    unescaped = None
);
