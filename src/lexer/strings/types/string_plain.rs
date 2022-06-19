use std::ops::ControlFlow;

use crate::lexer::{
    buffer::BufferWithCursor,
    strings::{
        action::StringExtendAction,
        handlers::{handle_eof, handle_string_end},
        literal::StringLiteralExtend,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringPlain {
    ends_with: u8,
}

impl StringPlain {
    pub(crate) fn new(ends_with: u8) -> Self {
        Self { ends_with }
    }
}

impl<'a> StringLiteralExtend<'a> for StringPlain {
    fn extend(
        &mut self,
        buffer: &mut BufferWithCursor<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<StringExtendAction<'a>> {
        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;
            handle_string_end(self.ends_with, buffer, start)?;
            buffer.skip_byte()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::{
        string_content::StringContent,
        strings::{test_helpers::*, StringLiteral},
    };

    assert_emits_token!(
        test = test_rcurly_with_no_interp_support,
        literal = StringLiteral::StringPlain(StringPlain::new(b'\'')),
        input = b"}",
        token = token!(tSTRING_CONTENT(StringContent::from(b"}")), 0, 1),
        pre = |_| {},
        post = |_| {}
    );

    #[test]
    fn test_string_plain_non_interp() {
        use crate::{lexer::Lexer, token::token};
        let mut lexer = Lexer::new(b"'foo'");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                token!(tSTRING_BEG, 0, 1),
                token!(tSTRING_CONTENT(StringContent::from(b"foo")), 1, 4),
                token!(tSTRING_END, 4, 5),
                token!(tEOF, 5, 5)
            ]
        );
    }

    assert_emits_string_end!(
        literal = StringLiteral::StringPlain(StringPlain::new(b'\'')),
        input = b"'"
    );
}
