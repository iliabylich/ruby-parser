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
    starts_with: u8,
    ends_with: u8,
}

impl StringPlain {
    pub(crate) fn new(starts_with: u8, ends_with: u8) -> Self {
        Self {
            starts_with,
            ends_with,
        }
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
        literal = StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')),
        input = b"}",
        token = token!(tSTRING_CONTENT(StringContent::from(b"}")), 0, 1),
        pre = |_| {},
        post = |action: StringExtendAction| {
            assert_eq!(
                action,
                StringExtendAction::EmitEOF { at: 1 },
                "2nd action daction doesn't match"
            )
        }
    );

    #[test]
    fn test_string_plain() {
        use crate::{lexer::Lexer, token::token};
        let mut lexer = Lexer::new(b"'foo\\\nbar'");
        assert_eq!(
            lexer.tokenize_until_eof(),
            vec![
                token!(tSTRING_BEG, 0, 1),
                token!(tSTRING_CONTENT(StringContent::from(b"foo\\\nbar")), 1, 9),
                token!(tSTRING_END, 9, 10),
                token!(tEOF, 10, 10)
            ]
        );
    }

    assert_emits_string_end!(
        literal = StringLiteral::StringPlain(StringPlain::new(b'\'', b'\'')),
        input = b"'"
    );
}
