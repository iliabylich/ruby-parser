use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_method(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("method definition")
            .or_else(|| {
                let ((def_t, name_t), args, body, end_t) = self
                    .all_of("instance method definition")
                    .and(|| self.try_defn_head())
                    .and(|| self.try_f_arglist())
                    .and(|| self.try_bodystmt())
                    .and(|| self.try_k_end())
                    .stop()?;

                todo!("{:?} {:?} {:?} {:?} {:?}", def_t, name_t, args, body, end_t)
            })
            .or_else(|| {
                let ((def_t, singleton, op_t, name_t), args, body, end_t) = self
                    .all_of("singleton method definition")
                    .and(|| self.try_defs_head())
                    .and(|| self.try_f_arglist())
                    .and(|| self.try_bodystmt())
                    .and(|| self.try_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?} {:?} {:?}",
                    def_t,
                    singleton,
                    op_t,
                    name_t,
                    args,
                    body,
                    end_t
                )
            })
            .stop()
    }

    pub(crate) fn try_defn_head(&mut self) -> ParseResult<(Token, Token)> {
        self.all_of("instance method definition start")
            .and(|| self.try_k_def())
            .and(|| self.try_def_name())
            .stop()
    }

    pub(crate) fn try_defs_head(&mut self) -> ParseResult<(Token, Box<Node>, Token, Token)> {
        self.all_of("singleton method definition start")
            .and(|| self.try_k_def())
            .and(|| self.try_singleton())
            .and(|| self.try_dot_or_colon())
            .and(|| self.try_def_name())
            .stop()
    }

    fn try_k_def(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kDEF)
    }

    fn try_f_arglist(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_f_arglist")
    }

    fn try_singleton(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("singleton")
            .or_else(|| self.try_var_ref())
            .or_else(|| {
                let (lparen_t, expr, rparen_t) = self
                    .all_of("(expr)")
                    .and(|| self.try_token(TokenKind::tLPAREN))
                    .and(|| self.try_expr())
                    .and(|| self.try_rparen())
                    .stop()?;
                todo!("{:?} {:?} {:?}", lparen_t, expr, rparen_t)
            })
            .stop()
    }
}
