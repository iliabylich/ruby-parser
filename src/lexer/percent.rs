use crate::lexer::buffer::Buffer;
use crate::token::{token, Token};

use crate::lexer::strings::literal::StringLiteral;

pub(crate) fn parse_percent<'a>(
    buffer: &mut Buffer<'a>,
    curly_level: usize,
) -> (Option<StringLiteral<'a>>, Token) {
    let start = buffer.pos();
    buffer.skip_byte();

    let mut ends_with;
    let literal_type;

    if let Some(c) = buffer.current_byte() {
        if !c.is_ascii_alphanumeric() {
            if c.is_ascii() {
                // %< or something similar to punctuation byte
                ends_with = buffer.slice(buffer.pos(), buffer.pos() + 1);
                literal_type = b'Q';
                buffer.skip_byte();
            } else {
                todo!("percent_unknown")
            }
        } else {
            literal_type = c;
            buffer.skip_byte();

            if let Some(c) = buffer.current_byte() {
                if c.is_ascii_alphabetic() {
                    todo!("percent_unknown")
                }
                ends_with = buffer.slice(buffer.pos(), buffer.pos() + 1);
                buffer.skip_byte();
            } else {
                todo!("percent_unterminated")
            }
        }
    } else {
        todo!("percent_unterminated")
    }

    match ends_with {
        b"(" => ends_with = b")",
        b"[" => ends_with = b"]",
        b"{" => ends_with = b"}",
        b"<" => ends_with = b">",
        _ => {}
    }

    let end = buffer.pos();
    let token;

    let literal;

    match literal_type {
        b'Q' => {
            token = token!(tSTRING_BEG, start, end);
            literal = StringLiteral::string()
                .with_ending(ends_with)
                .with_interpolation_support(true);
        }
        b'q' => {
            token = token!(tSTRING_BEG, start, end);
            literal = StringLiteral::string()
                .with_ending(ends_with)
                .with_interpolation_support(false);
        }

        b'W' => {
            token = token!(tWORDS_BEG, start, end);
            literal = StringLiteral::array()
                .with_ending(ends_with)
                .with_interpolation_support(true);
        }
        b'w' => {
            token = token!(tWORDS_BEG, start, end);
            literal = StringLiteral::array()
                .with_ending(ends_with)
                .with_interpolation_support(false);
        }

        b'I' => {
            token = token!(tSYMBOLS_BEG, start, end);
            literal = StringLiteral::array()
                .with_ending(ends_with)
                .with_interpolation_support(true);
        }
        b'i' => {
            token = token!(tSYMBOLS_BEG, start, end);
            literal = StringLiteral::array()
                .with_ending(ends_with)
                .with_interpolation_support(false);
        }

        b'x' => {
            token = token!(tXSTRING_BEG, start, end);
            literal = StringLiteral::string()
                .with_ending(ends_with)
                .with_interpolation_support(true);
        }

        b'r' => {
            token = token!(tREGEXP_BEG, start, end);
            literal = StringLiteral::regexp()
                .with_ending(ends_with)
                .with_interpolation_support(true);
        }

        b's' => {
            token = token!(tSYMBEG, start, end);
            literal = StringLiteral::string()
                .with_ending(ends_with)
                .with_interpolation_support(false);
        }

        _ => todo!("percent_unknown"),
    };

    (Some(literal.with_curly_level(curly_level)), token)
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
            type = $type:ident,
            token = $token:ident,
            ends_with = $ends_with:expr,
            with_interpolation_support = $interpolation_support:expr
        ) => {
            #[test]
            fn $name() {
                let len = $input.len();

                let mut lexer = Lexer::new($input);
                let token = lexer.next_token();
                assert_eq!(token, token!($token, 0, len));

                assert_eq!(
                    lexer.string_literals.size(),
                    1,
                    "expected a string literal to be pushed"
                );
                let literal = lexer.string_literals.last().unwrap();
                assert_eq!(
                    literal,
                    StringLiteral::$type()
                        .with_interpolation_support($interpolation_support)
                        .with_ending($ends_with)
                );

                assert_eq!(lexer.next_token(), token!(tEOF, len, len));
            }
        };
    }

    // $PUNCTUATION
    test_string_literal_start!(
        name = test_tPERCENT_tLPAREN,
        input = b"%(",
        type = string,
        token = tSTRING_BEG,
        ends_with = b")",
        with_interpolation_support = true
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLBRACK,
        input = b"%[",
        type = string,
        token = tSTRING_BEG,
        ends_with = b"]",
        with_interpolation_support = true
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLCURLY,
        input = b"%{",
        type = string,
        token = tSTRING_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );
    test_string_literal_start!(
        name = test_tPERCENT_tLT,
        input = b"%<",
        type = string,
        token = tSTRING_BEG,
        ends_with = b">",
        with_interpolation_support = true
    );
    test_string_literal_start!(
        name = test_tPERCENT_tPIPE,
        input = b"%|",
        type = string,
        token = tSTRING_BEG,
        ends_with = b"|",
        with_interpolation_support = true
    );

    // %Q
    test_string_literal_start!(
        name = test_tPERCENT_q_upper,
        input = b"%Q{",
        type = string,
        token = tSTRING_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );

    // %q
    test_string_literal_start!(
        name = test_tPERCENT_q_lower,
        input = b"%q{",
        type = string,
        token = tSTRING_BEG,
        ends_with = b"}",
        with_interpolation_support = false
    );

    // %W
    test_string_literal_start!(
        name = test_tPERCENT_w_upper,
        input = b"%W{",
        type = array,
        token = tWORDS_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );

    // %w
    test_string_literal_start!(
        name = test_tPERCENT_w_lower,
        input = b"%w{",
        type = array,
        token = tWORDS_BEG,
        ends_with = b"}",
        with_interpolation_support = false
    );

    // %I
    test_string_literal_start!(
        name = test_tPERCENT_i_upper,
        input = b"%I{",
        type = array,
        token = tSYMBOLS_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );

    // %i
    test_string_literal_start!(
        name = test_tPERCENT_i_lower,
        input = b"%i{",
        type = array,
        token = tSYMBOLS_BEG,
        ends_with = b"}",
        with_interpolation_support = false
    );

    // %x
    test_string_literal_start!(
        name = test_tPERCENT_x_lower,
        input = b"%x{",
        type = string,
        token = tXSTRING_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );

    // %r
    test_string_literal_start!(
        name = test_tPERCENT_r_lower,
        input = b"%r{",
        type = regexp,
        token = tREGEXP_BEG,
        ends_with = b"}",
        with_interpolation_support = true
    );

    // %s
    test_string_literal_start!(
        name = test_tPERCENT_s_lower,
        input = b"%s{",
        type = string,
        token = tSYMBEG,
        ends_with = b"}",
        with_interpolation_support = false
    );
}
