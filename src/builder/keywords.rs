use crate::{
    builder::{
        helpers::{maybe_loc, maybe_node_expr},
        Builder,
    },
    nodes::{Break, Defined, Next, Redo, Retry, Return, Super, Yield, ZSuper},
    Node, Token,
};

#[derive(Debug, PartialEq)]
pub(crate) enum KeywordCmd {
    Break,
    Defined,
    Next,
    Redo,
    Retry,
    Return,
    Super,
    Yield,
    Zsuper,
}

impl Builder {
    pub(crate) fn keyword_cmd(
        type_: KeywordCmd,
        keyword_t: Token,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,
    ) -> Box<Node> {
        let keyword_l = keyword_t.loc;

        match type_ {
            KeywordCmd::Yield
                if !args.is_empty() && matches!(args.last(), Some(Node::BlockPass(_))) =>
            {
                // self.error(DiagnosticMessage::BlockGivenToYield {}, &keyword_l);
            }
            KeywordCmd::Yield | KeywordCmd::Super => {
                // self.rewrite_hash_args_to_kwargs(&mut args);
            }
            _ => {}
        }

        let begin_l = maybe_loc(&lparen_t);
        let end_l = maybe_loc(&rparen_t);

        let expr_end_l = end_l
            .or_else(|| maybe_node_expr(&args.last()))
            .unwrap_or(keyword_l);

        let expression_l = keyword_l.join(&expr_end_l);

        let result = match type_ {
            KeywordCmd::Break => Node::Break(Break {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Defined => Node::Defined(Defined {
                value: Box::new(args.into_iter().next().unwrap()),
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Next => Node::Next(Next {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Redo => Node::Redo(Redo { expression_l }),
            KeywordCmd::Retry => Node::Retry(Retry { expression_l }),
            KeywordCmd::Return => Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }),
            KeywordCmd::Super => Node::Super(Super {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Yield => Node::Yield(Yield {
                args,
                keyword_l,
                begin_l,
                end_l,
                expression_l,
            }),
            KeywordCmd::Zsuper => Node::ZSuper(ZSuper { expression_l }),
        };

        Box::new(result)
    }
}
