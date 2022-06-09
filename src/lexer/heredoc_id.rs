use crate::{
    lexer::{
        buffer::{Buffer, Lookahead, LookaheadResult},
        ident::Ident,
    },
    token::{Loc, Token, TokenValue},
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct HeredocId {
    pub(crate) token: Token,
    pub(crate) id: (usize, usize),
    pub(crate) interp: bool,
    pub(crate) indent: bool,
}

impl<'a> Lookahead<'a> for HeredocId {
    type Output = Option<Self>;

    fn lookahead(buffer: &Buffer<'a>, mut start: usize) -> Self::Output {
        // We are at `<<`
        let heredoc_start = start;
        // consume `<<`
        start += 2;

        let mut id_start = start;
        let token_value;
        let indent;
        let interp;
        let quote;

        // Check if there's a `-`/`~` sign
        match buffer.byte_at(id_start) {
            Some(b'~') => {
                id_start += 1;
                indent = true;
            }
            Some(b'-') => {
                id_start += 1;
                indent = false;
            }
            _ => {
                indent = false;
            }
        }

        // Check if there's a ' or " or ` around heredoc id
        match buffer.byte_at(id_start) {
            Some(b'\'') => {
                token_value = TokenValue::tSTRING_BEG;
                interp = false;
                id_start += 1;
                quote = Some(b'\'');
            }
            Some(b'"') => {
                token_value = TokenValue::tDSTRING_BEG;
                interp = true;
                id_start += 1;
                quote = Some(b'"');
            }
            Some(b'`') => {
                token_value = TokenValue::tXSTRING_BEG;
                interp = true;
                id_start += 1;
                quote = Some(b'`');
            }
            _ => {
                token_value = TokenValue::tDSTRING_BEG;
                interp = true;
                quote = None;
            }
        }

        let id_end;
        let heredoc_end;

        if let Some(quote) = quote {
            // just read until quote
            let mut pos = id_start;
            loop {
                match buffer.byte_at(pos) {
                    None | Some(b'\r') | Some(b'\n') => {
                        // TODO: report unterminated heredoc id
                        return None;
                    }
                    Some(byte) if byte == quote => {
                        id_end = pos;
                        heredoc_end = pos + 1;
                        break;
                    }
                    _ => pos += 1,
                }
            }
        } else {
            // no quote, so read an identifier
            match Ident::lookahead(buffer, id_start) {
                LookaheadResult::None => {
                    // No valid chars to construct a heredoc ID,
                    // so this is probably just an tLSHIFT that is dispatched by a Lexer
                    return None;
                }
                LookaheadResult::Some { length } => {
                    heredoc_end = id_start + length;
                    id_end = heredoc_end;
                }
            }
        }

        let token = Token(token_value, Loc(heredoc_start, heredoc_end));

        Some(Self {
            token,
            id: (id_start, id_end),
            indent,
            interp,
        })
    }
}

impl HeredocId {
    pub(crate) fn parse(buffer: &mut Buffer) -> Option<Self> {
        let mut heredoc_id = Self::lookahead(buffer, buffer.pos());
        if let Some(heredoc_id) = heredoc_id.as_mut() {
            buffer.set_pos(heredoc_id.token.loc().end());
        }
        heredoc_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token;

    macro_rules! assert_heredoc_id {
        (
            test = $test:ident,
            input = $input:expr,
            output = $output:expr
        ) => {
            #[test]
            fn $test() {
                let mut buffer = Buffer::new($input);
                let output = HeredocId::parse(&mut buffer);
                assert_eq!(output, $output);
            }
        };
    }

    // prefixes
    assert_heredoc_id!(
        test = test_heredoc_id_minus,
        input = b"<<-HERE",
        output = Some(HeredocId {
            token: token!(tDSTRING_BEG, 0, 7),
            id: (3, 7),
            interp: true,
            indent: false
        })
    );
    assert_heredoc_id!(
        test = test_heredoc_id_tilde,
        input = b"<<~HERE",
        output = Some(HeredocId {
            token: token!(tDSTRING_BEG, 0, 7),
            id: (3, 7),
            interp: true,
            indent: true
        })
    );

    // quotes
    assert_heredoc_id!(
        test = test_heredoc_id_squote,
        input = b"<<-'HERE'",
        output = Some(HeredocId {
            token: token!(tSTRING_BEG, 0, 9),
            id: (4, 8),
            interp: false,
            indent: false
        })
    );
    assert_heredoc_id!(
        test = test_heredoc_id_dquote,
        input = b"<<-\"HERE\"",
        output = Some(HeredocId {
            token: token!(tDSTRING_BEG, 0, 9),
            id: (4, 8),
            interp: true,
            indent: false
        })
    );
    assert_heredoc_id!(
        test = test_heredoc_id_xquote,
        input = b"<<-`HERE`",
        output = Some(HeredocId {
            token: token!(tXSTRING_BEG, 0, 9),
            id: (4, 8),
            interp: true,
            indent: false
        })
    );

    // unterminated heredoc IDs
    assert_heredoc_id!(
        test = test_heredoc_id_quote_unterminated,
        input = b"<<-'HERE",
        output = None
    );
    assert_heredoc_id!(
        test = test_heredoc_id_no_quote_unterminated,
        input = b"<<-)",
        output = None
    );
}
