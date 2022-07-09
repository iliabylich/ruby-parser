use crate::{
    lexer::{
        buffer::BufferWithCursor,
        strings::{literal::StringLiteral, types::*},
    },
    loc::loc,
    token::{token, Token},
};

pub(crate) fn parse_percent(
    buffer: &mut BufferWithCursor,
    curly_level: usize,
) -> (Option<StringLiteral>, Token) {
    let start = buffer.pos();
    buffer.skip_byte();

    let starts_with;
    let literal_type;

    if let Some(c) = buffer.current_byte() {
        if !c.is_ascii_alphanumeric() {
            if c.is_ascii() {
                // %< or something similar to punctuation byte
                starts_with = c;
                literal_type = b'Q';
                buffer.skip_byte();
            } else {
                panic!("percent_unknown")
            }
        } else {
            literal_type = c;
            buffer.skip_byte();

            if let Some(c) = buffer.current_byte() {
                if c.is_ascii_alphabetic() {
                    panic!("percent_unknown")
                }
                starts_with = c;
                buffer.skip_byte();
            } else {
                panic!("percent_unterminated")
            }
        }
    } else {
        panic!("percent_unterminated")
    }

    let ends_with = match starts_with {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => starts_with,
    };

    let end = buffer.pos();
    let token;

    let literal;

    match literal_type {
        b'Q' => {
            token = token!(tSTRING_BEG, loc!(start, end));
            literal = StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(curly_level),
                starts_with,
                ends_with,
            ));
        }
        b'q' => {
            token = token!(tSTRING_BEG, loc!(start, end));
            literal = StringLiteral::StringPlain(StringPlain::new(starts_with, ends_with));
        }

        b'W' => {
            token = token!(tWORDS_BEG, loc!(start, end));
            literal = StringLiteral::Array(Array::new(true, starts_with, ends_with, curly_level));
        }
        b'w' => {
            token = token!(tWORDS_BEG, loc!(start, end));
            literal = StringLiteral::Array(Array::new(false, starts_with, ends_with, curly_level));
        }

        b'I' => {
            token = token!(tSYMBOLS_BEG, loc!(start, end));
            literal = StringLiteral::Array(Array::new(true, starts_with, ends_with, curly_level));
        }
        b'i' => {
            token = token!(tSYMBOLS_BEG, loc!(start, end));
            literal = StringLiteral::Array(Array::new(false, starts_with, ends_with, curly_level));
        }

        b'x' => {
            token = token!(tXSTRING_BEG, loc!(start, end));
            literal = StringLiteral::StringInterp(StringInterp::new(
                Interpolation::new(curly_level),
                starts_with,
                ends_with,
            ));
        }

        b'r' => {
            token = token!(tREGEXP_BEG, loc!(start, end));
            literal = StringLiteral::Regexp(Regexp::new(starts_with, ends_with, curly_level));
        }
        b's' => {
            token = token!(tSYMBEG, loc!(start, end));
            literal = StringLiteral::StringPlain(StringPlain::new(starts_with, ends_with));
        }

        _ => panic!("percent_unknown"),
    };

    (Some(literal), token)
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    macro_rules! test_string_literal_start {
        (
            name = $name:ident,
            input = $input:expr,
            token = $token:ident,
            literal = $literal:expr
        ) => {
            #[test]
            fn $name() {
                let len = $input.len();

                let (mut lexer, _state) = Lexer::new_managed($input);
                let token = lexer.next_token();
                assert_eq!(token, token!($token, loc!(0, len)));

                assert_eq!(
                    lexer.string_literals().size(),
                    1,
                    "expected a string literal to be pushed"
                );
                let literal = lexer.string_literals().last().unwrap();
                assert_eq!(literal, &$literal);

                assert_eq!(lexer.next_token(), token!(tEOF, loc!(len, len)));
            }
        };
    }

    // $PUNCTUATION
    test_string_literal_start!(
        name = test_tPERCENT_tLPAREN,
        input = b"%(",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'(', b')'))
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLBRACK,
        input = b"%[",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'[', b']'))
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLCURLY,
        input = b"%{",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'{', b'}'))
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLT,
        input = b"%<",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'<', b'>'))
    );
    test_string_literal_start!(
        name = test_tPERCENT_tPIPE,
        input = b"%|",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'|', b'|'))
    );

    // %Q
    test_string_literal_start!(
        name = test_tPERCENT_q_upper,
        input = b"%Q{",
        token = tSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'{', b'}'))
    );

    // %q
    test_string_literal_start!(
        name = test_tPERCENT_q_lower,
        input = b"%q{",
        token = tSTRING_BEG,
        literal = StringLiteral::StringPlain(StringPlain::new(b'{', b'}'))
    );

    // %W
    test_string_literal_start!(
        name = test_tPERCENT_w_upper,
        input = b"%W{",
        token = tWORDS_BEG,
        literal = StringLiteral::Array(Array::new(true, b'{', b'}', 0))
    );

    // %w
    test_string_literal_start!(
        name = test_tPERCENT_w_lower,
        input = b"%w{",
        token = tWORDS_BEG,
        literal = StringLiteral::Array(Array::new(false, b'{', b'}', 0))
    );

    // %I
    test_string_literal_start!(
        name = test_tPERCENT_i_upper,
        input = b"%I{",
        token = tSYMBOLS_BEG,
        literal = StringLiteral::Array(Array::new(true, b'{', b'}', 0))
    );

    // %i
    test_string_literal_start!(
        name = test_tPERCENT_i_lower,
        input = b"%i{",
        token = tSYMBOLS_BEG,
        literal = StringLiteral::Array(Array::new(false, b'{', b'}', 0))
    );

    // %x
    test_string_literal_start!(
        name = test_tPERCENT_x_lower,
        input = b"%x{",
        token = tXSTRING_BEG,
        literal = StringLiteral::StringInterp(StringInterp::new(Interpolation::new(0), b'{', b'}'))
    );

    // %r
    test_string_literal_start!(
        name = test_tPERCENT_r_lower,
        input = b"%r{",
        token = tREGEXP_BEG,
        literal = StringLiteral::Regexp(Regexp::new(b'{', b'}', 0))
    );

    // %s
    test_string_literal_start!(
        name = test_tPERCENT_s_lower,
        input = b"%s{",
        token = tSYMBEG,
        literal = StringLiteral::StringPlain(StringPlain::new(b'{', b'}'))
    );
}
