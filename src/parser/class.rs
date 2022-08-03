use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn parse_class(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("class definition")
            .or_else(|| {
                let (class_t, cpath, superclass, body, end_t) = self
                    .all_of("normal class definition")
                    .and(|| self.parse_k_class())
                    .and(|| self.parse_cpath())
                    .and(|| self.try_superclass())
                    .and(|| self.try_bodystmt())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    class_t,
                    cpath,
                    superclass,
                    body,
                    end_t
                )
            })
            .or_else(|| {
                let (klass_t, lshift_t, expr, _term, body, end_t) = self
                    .all_of("singleton class")
                    .and(|| self.parse_k_class())
                    .and(|| self.try_token(TokenKind::tLSHFT))
                    .and(|| self.parse_expr())
                    .and(|| self.parse_term())
                    .and(|| self.try_bodystmt())
                    .and(|| self.parse_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?} {:?}",
                    klass_t,
                    lshift_t,
                    expr,
                    _term,
                    body,
                    end_t
                )
            })
            .stop()
    }

    pub(crate) fn parse_cpath(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.parse_cpath")
    }

    fn try_superclass(&mut self) -> ParseResult<Option<Box<Node>>> {
        todo!("parser.try_superclass")
    }

    fn parse_k_class(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kCLASS)
    }
}
