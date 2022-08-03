use crate::{
    parser::{ParseResult, Parser},
    token::{Token, TokenKind},
    Node,
};

impl Parser {
    pub(crate) fn try_case(&mut self) -> ParseResult<Box<Node>> {
        self.one_of("case expr")
            .or_else(|| {
                let (case_t, expr, _terms, body, end_t) = self
                    .all_of("k_case expr_value opt_terms case_body k_end")
                    .and(|| self.try_k_case())
                    .and(|| self.try_expr_value())
                    .and(|| self.try_opt_terms())
                    .and(|| self.try_case_body())
                    .and(|| self.try_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    case_t,
                    expr,
                    _terms,
                    body,
                    end_t
                )
            })
            .or_else(|| {
                let (case_t, _opt_terms, body, end_t) = self
                    .all_of("k_case opt_terms case_body k_end")
                    .and(|| self.try_k_case())
                    .and(|| self.try_opt_terms())
                    .and(|| self.try_case_body())
                    .and(|| self.try_k_end())
                    .stop()?;

                todo!("{:?} {:?} {:?} {:?}", case_t, _opt_terms, body, end_t)
            })
            .or_else(|| {
                let (case_t, expr, _opt_terms, p_case_body, end_t) = self
                    .all_of("k_case expr_value opt_terms p_case_body k_end")
                    .and(|| self.try_k_case())
                    .and(|| self.try_expr_value())
                    .and(|| self.try_opt_terms())
                    .and(|| self.try_p_case_body())
                    .and(|| self.try_k_end())
                    .stop()?;

                todo!(
                    "{:?} {:?} {:?} {:?} {:?}",
                    case_t,
                    expr,
                    _opt_terms,
                    p_case_body,
                    end_t
                )
            })
            .stop()
    }

    fn try_case_args(&mut self) {
        todo!("parser.try_case_args")
    }
    fn try_case_body(&mut self) -> ParseResult<Box<Node>> {
        todo!("parser.try_case_body")
    }
    fn try_cases(&mut self) {
        todo!("parser.try_cases")
    }

    fn try_k_case(&mut self) -> ParseResult<Token> {
        self.try_token(TokenKind::kCASE)
    }
}
