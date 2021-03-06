use crate::{
    buffer::{Buffer, BufferWithCursor},
    lexer::ident::Ident,
    loc::loc,
    token::{token, Token, TokenKind},
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct HeredocId {
    pub(crate) token: Token,
    pub(crate) id: (usize, usize),
    pub(crate) interpolated: bool,
    pub(crate) squiggly: bool,
}

pub(crate) enum HeredocIdError {
    UnterminatedHeredocId,
}

impl HeredocId {
    fn lookahead(buffer: &Buffer, mut start: usize) -> Result<Option<Self>, HeredocIdError> {
        // We are at `<<`
        let heredoc_start = start;
        // consume `<<`
        start += 2;

        let mut id_start = start;
        let token_value;
        let squiggly;
        let interpolated;
        let quote;

        // Check if there's a `-`/`~` sign
        match buffer.byte_at(id_start) {
            Some(b'~') => {
                id_start += 1;
                squiggly = true;
            }
            Some(b'-') => {
                id_start += 1;
                squiggly = false;
            }
            _ => {
                squiggly = false;
            }
        }

        // Check if there's a ' or " or ` around heredoc id
        match buffer.byte_at(id_start) {
            Some(b'\'') => {
                token_value = TokenKind::tSTRING_BEG;
                interpolated = false;
                id_start += 1;
                quote = Some(b'\'');
            }
            Some(b'"') => {
                token_value = TokenKind::tDSTRING_BEG;
                interpolated = true;
                id_start += 1;
                quote = Some(b'"');
            }
            Some(b'`') => {
                token_value = TokenKind::tXSTRING_BEG;
                interpolated = true;
                id_start += 1;
                quote = Some(b'`');
            }
            _ => {
                token_value = TokenKind::tDSTRING_BEG;
                interpolated = true;
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
                        return Err(HeredocIdError::UnterminatedHeredocId);
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
                None => {
                    // No valid chars to construct a heredoc ID,
                    // so this is probably just an tLSHIFT that is dispatched by a Lexer
                    return Ok(None);
                }
                Some(Ident { length }) => {
                    heredoc_end = id_start + length;
                    id_end = heredoc_end;
                }
            }
        }

        let token = token!(token_value, loc!(heredoc_start, heredoc_end));

        Ok(Some(Self {
            token,
            id: (id_start, id_end),
            squiggly,
            interpolated,
        }))
    }
}

impl HeredocId {
    pub(crate) fn parse(buffer: &mut BufferWithCursor) -> Option<Self> {
        let start = buffer.pos();
        let heredoc_id = Self::lookahead(buffer.for_lookahead(), start);
        match heredoc_id {
            Ok(heredoc_id) => {
                let heredoc_id = heredoc_id?;
                buffer.set_pos(heredoc_id.token.loc.end);
                Some(heredoc_id)
            }
            Err(HeredocIdError::UnterminatedHeredocId) => {
                // TODO: report unterminated heredoc ID
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::token;

    macro_rules! assert_parses_heredoc_id {
        (
            $input:expr,
            $output:expr
        ) => {{
            let mut buffer = BufferWithCursor::new($input);
            let output = HeredocId::parse(&mut buffer);
            assert_eq!(output, $output);
        }};
    }

    // prefixes
    #[test]
    fn test_heredoc_id_minus() {
        assert_parses_heredoc_id!(
            b"<<-HERE",
            Some(HeredocId {
                token: token!(tDSTRING_BEG, loc!(0, 7)),
                id: (3, 7),
                interpolated: true,
                squiggly: false
            })
        );
    }

    #[test]
    fn test_heredoc_id_tilde() {
        assert_parses_heredoc_id!(
            b"<<~HERE",
            Some(HeredocId {
                token: token!(tDSTRING_BEG, loc!(0, 7)),
                id: (3, 7),
                interpolated: true,
                squiggly: true
            })
        );
    }

    // quotes
    #[test]
    fn test_heredoc_id_squote() {
        assert_parses_heredoc_id!(
            b"<<-'HERE'",
            Some(HeredocId {
                token: token!(tSTRING_BEG, loc!(0, 9)),
                id: (4, 8),
                interpolated: false,
                squiggly: false
            })
        );
    }
    #[test]
    fn test_heredoc_id_dquote() {
        assert_parses_heredoc_id!(
            b"<<-\"HERE\"",
            Some(HeredocId {
                token: token!(tDSTRING_BEG, loc!(0, 9)),
                id: (4, 8),
                interpolated: true,
                squiggly: false
            })
        );
    }
    #[test]
    fn test_heredoc_id_xquote() {
        assert_parses_heredoc_id!(
            b"<<-`HERE`",
            Some(HeredocId {
                token: token!(tXSTRING_BEG, loc!(0, 9)),
                id: (4, 8),
                interpolated: true,
                squiggly: false
            })
        );
    }

    // unterminated heredoc IDs
    #[test]
    fn test_heredoc_id_quote_unterminated() {
        assert_parses_heredoc_id!(b"<<-'HERE", None);
    }
    #[test]
    fn test_heredoc_id_no_quote_unterminated() {
        assert_parses_heredoc_id!(b"<<-)", None);
    }
}
