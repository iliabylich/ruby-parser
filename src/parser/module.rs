use crate::{
    builder::Builder,
    parser::{macros::all_of, ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_module(&mut self) -> ParseResult<Box<Node>> {
        let (module_t, name, body, end_t) = all_of!(
            "module definition",
            self.parse_k_module(),
            self.parse_cpath(),
            self.try_bodystmt(),
            self.parse_k_end(),
        )?;

        Ok(Builder::def_module(module_t, name, body, end_t))
    }

    fn parse_k_module(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kMODULE)
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::assert_parses;

    #[test]
    fn test_module() {
        assert_parses!(
            parse_module,
            b"module Foo::Bar; A = 1; end",
            r#"
            "#
        )
    }
}
