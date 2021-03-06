use crate::{
    builder::Constructor,
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl<C> Parser<C>
where
    C: Constructor,
{
    pub(crate) fn try_class(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("class definition")
            .or_else(|| {
                let (class_t, cpath, superclass, body, end_t) = self
                    .all_of("normal class definition")
                    .and(|| self.try_k_class())
                    .and(|| self.try_cpath())
                    .and(|| self.try_superclass())
                    .and(|| self.try_bodystmt())
                    .and(|| self.try_k_end())
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
                    .and(|| self.try_k_class())
                    .and(|| self.try_token(TokenKind::tLSHFT))
                    .and(|| self.try_expr())
                    .and(|| self.try_term())
                    .and(|| self.try_bodystmt())
                    .and(|| self.try_k_end())
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

    pub(crate) fn try_cpath(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_cpath")
    }

    fn try_superclass(&mut self) -> ParseResult<Option<Box<Node>>> {
        todo!("parser.try_superclass")
    }

    fn try_k_class(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kCLASS)
    }
}
