use crate::{
    buffer::Buffer,
    builder::{builders::helpers::*, Builder, Constructor},
    nodes::*,
    string_content::StringContent,
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
    pub(crate) fn forwarded_args() -> Box<Node> {
        todo!("builder.forwarded_args")
    }
    pub(crate) fn call_method() -> Box<Node> {
        todo!("builder.call_method")
    }
    pub(crate) fn call_lambda() -> Box<Node> {
        todo!("builder.call_lambda")
    }
    pub(crate) fn block() -> Box<Node> {
        todo!("builder.block")
    }
    pub(crate) fn block_pass() -> Box<Node> {
        todo!("builder.block_pass")
    }
    pub(crate) fn attr_asgn() -> Box<Node> {
        todo!("builder.attr_asgn")
    }
    pub(crate) fn index() -> Box<Node> {
        todo!("builder.index")
    }
    pub(crate) fn index_asgn() -> Box<Node> {
        todo!("builder.index_asgn")
    }
    pub(crate) fn binary_op(
        receiver: Box<Node>,
        operator_t: Token,
        arg: Box<Node>,
        buffer: &Buffer,
    ) -> Box<Node> {
        // TODO: check receiver is value_expr
        // TODO: check arg is value_expr

        let selector_l = Some(operator_t.loc);
        let expression_l = receiver.expression().join(arg.expression());

        Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name: string_value(operator_t.loc, buffer),
            args: vec![*arg],
            dot_l: None,
            selector_l,
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        }))
    }
    pub(crate) fn match_op(receiver: Box<Node>, match_t: Token, arg: Box<Node>) -> Box<Node> {
        // TODO: check receiver is value_expr
        // TODO: check arg is value_expr

        let selector_l = match_t.loc;
        let expression_l = receiver.expression().join(arg.expression());

        let result = match static_regexp_captures(&receiver) {
            Some(captures) => {
                // TODO: declare all captures in static env
                // for capture in captures {
                //     static_env.declare(&capture);
                // }

                Node::MatchWithLvasgn(MatchWithLvasgn {
                    re: receiver,
                    value: arg,
                    operator_l: selector_l,
                    expression_l,
                })
            }
            None => Node::Send(Send {
                recv: Some(receiver),
                method_name: StringContent::from("=~"),
                args: vec![*arg],
                dot_l: None,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }),
        };

        Box::new(result)
    }
    pub(crate) fn unary_op() -> Box<Node> {
        todo!("builder.unary_op")
    }
    pub(crate) fn not_op(
        not_t: Token,
        begin_t: Option<Token>,
        receiver: Option<Box<Node>>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!("builder.not_op")
    }
}
