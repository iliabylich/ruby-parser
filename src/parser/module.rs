use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_module(&mut self) -> ParseResult<Box<Node>> {
        let (module_t, cpath, body, end_t) = self
            .all_of("module definition")
            .and(|| self.parse_k_module())
            .and(|| self.parse_cpath())
            .and(|| self.try_bodystmt())
            .and(|| self.parse_k_end())
            .stop()?;

        todo!("{:?} {:?} {:?} {:?}", module_t, cpath, body, end_t)
    }

    fn parse_k_module(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kMODULE)
    }
}
