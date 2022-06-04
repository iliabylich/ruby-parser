use std::ops::ControlFlow;

use crate::lexer::{
    buffer::Buffer,
    strings::{
        action::NextAction,
        handlers::{handle_eof, handle_next_action, handle_string_end},
        literal::StringLiteralExtend,
        types::HasNextAction,
    },
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct StringNoInterp {
    ends_with: u8,

    next_action: NextAction,
}

impl StringNoInterp {
    pub(crate) fn new(ends_with: u8) -> Self {
        Self {
            ends_with,

            next_action: NextAction::NoAction,
        }
    }
}

impl HasNextAction for StringNoInterp {
    fn next_action_mut(&mut self) -> &mut NextAction {
        &mut self.next_action
    }
}

impl<'a> StringLiteralExtend<'a> for StringNoInterp {
    fn extend(
        &mut self,
        buffer: &mut Buffer<'a>,
        _current_curly_nest: usize,
    ) -> ControlFlow<crate::lexer::strings::action::StringExtendAction> {
        handle_next_action(self)?;

        let start = buffer.pos();

        loop {
            handle_eof(buffer, start)?;
            handle_string_end(self, self.ends_with, buffer, start)?;
            buffer.skip_byte()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::strings::{test_helpers::*, StringLiteral};

    assert_emits_token!(
        test = test_rcurly_with_no_interp_support,
        literal = StringLiteral::StringNoInterp(StringNoInterp::new(b'\'')),
        input = b"}",
        token = token!(tSTRING_CONTENT, 0, 1),
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
                token!(tSTRING_CONTENT, 1, 4),
                token!(tSTRING_END, 4, 5),
                token!(tEOF, 5, 5)
            ]
        );
    }

    assert_emits_string_end!(
        literal = StringLiteral::StringNoInterp(StringNoInterp::new(b'\'')),
        input = b"'"
    );
}
