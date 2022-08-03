use crate::{
    buffer::Buffer,
    builder::{
        builders::helpers::{
            collection_map, maybe_boxed_node_expr, maybe_loc, maybe_node_expr, maybe_string_value,
            static_regexp_captures, string_value,
        },
        Builder,
    },
    nodes::{
        Begin, Block, BlockPass, Break, CSend, Index, IndexAsgn, Lambda, MatchWithLvasgn, Next,
        Nil, Numblock, Return, Send, Yield,
    },
    string_content::StringContent,
    token::{Token, TokenKind},
    Loc, Node,
};

pub(crate) enum ArgsType {
    Args(Option<Box<Node>>),
    Numargs(u8),
}

impl Builder {
    pub(crate) fn call_method(
        receiver: Option<Box<Node>>,
        dot_t: Option<Token>,
        selector_t: Option<Token>,
        lparen_t: Option<Token>,
        args: Vec<Node>,
        rparen_t: Option<Token>,
        buffer: &Buffer,
    ) -> Box<Node> {
        let begin_l = maybe_boxed_node_expr(&receiver)
            .or_else(|| maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute begin_l"));
        let end_l = maybe_loc(&rparen_t)
            .or_else(|| maybe_node_expr(&args.last()))
            .or_else(|| maybe_loc(&selector_t))
            .unwrap_or_else(|| unreachable!("can't compute end_l"));

        let expression_l = begin_l.join(&end_l);

        let dot_l = maybe_loc(&dot_t);
        let selector_l = maybe_loc(&selector_t);
        let begin_l = maybe_loc(&lparen_t);
        let end_l = maybe_loc(&rparen_t);

        let method_name = maybe_string_value(selector_l, buffer);
        let method_name = method_name.unwrap_or_else(|| StringContent::from("call"));

        // self.rewrite_hash_args_to_kwargs(&mut args);

        match call_type_for_dot(&dot_t) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                recv: receiver,
                method_name,
                args,
                dot_l,
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                recv: receiver.expect("csend node must have a receiver"),
                method_name,
                args,
                dot_l: dot_l.expect("csend node must have &."),
                selector_l,
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            })),
        }
    }

    pub(crate) fn call_lambda(lambda_t: Token) -> Box<Node> {
        Box::new(Node::Lambda(Lambda {
            expression_l: lambda_t.loc,
        }))
    }

    pub(crate) fn block(
        method_call: Box<Node>,
        begin_t: Token,
        block_args: ArgsType,
        body: Option<Box<Node>>,
        end_t: Token,
    ) -> Box<Node> {
        let block_body = body;

        let validate_block_and_block_arg = |args: &Vec<Node>| {
            if let Some(last_arg) = args.last() {
                match last_arg {
                    Node::BlockPass(_) | Node::ForwardedArgs(_) => {
                        // self.error(
                        //     DiagnosticMessage::BlockAndBlockArgGiven {},
                        //     last_arg.expression(),
                        // );
                    }
                    _ => {}
                }
            }
        };

        match &*method_call {
            Node::Yield(Yield { keyword_l, .. }) => {
                // self.error(DiagnosticMessage::BlockGivenToYield {}, keyword_l);
            }
            Node::Send(Send { args, .. }) => {
                validate_block_and_block_arg(args);
            }
            Node::CSend(CSend { args, .. }) => {
                validate_block_and_block_arg(args);
            }
            _ => {}
        }

        let rewrite_args_and_loc =
            |method_args: Vec<Node>,
             keyword_expression_l: Loc,
             block_args: ArgsType,
             block_body: Option<Box<Node>>| {
                // Code like "return foo 1 do end" is reduced in a weird sequence.
                // Here, method_call is actually (return).
                let actual_send = method_args.into_iter().next().unwrap();

                let begin_l = begin_t.loc;
                let end_l = end_t.loc;
                let expression_l = actual_send.expression().join(&end_l);

                let block = match block_args {
                    ArgsType::Args(args) => Node::Block(Block {
                        call: Box::new(actual_send),
                        args,
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        call: Box::new(actual_send),
                        numargs,
                        body: block_body.unwrap_or_else(|| {
                            Box::new(Node::Nil(Nil {
                                expression_l: Loc { start: 0, end: 0 },
                            }))
                        }),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };

                let expr_l = keyword_expression_l.join(block.expression());

                (vec![block], expr_l)
            };

        match &*method_call {
            Node::Send(_)
            | Node::CSend(_)
            | Node::Index(_)
            | Node::Super(_)
            | Node::ZSuper(_)
            | Node::Lambda(_) => {
                let begin_l = begin_t.loc;
                let end_l = end_t.loc;
                let expression_l = method_call.expression().join(&end_l);

                let result = match block_args {
                    ArgsType::Args(args) => Node::Block(Block {
                        call: method_call,
                        args,
                        body: block_body,
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                    ArgsType::Numargs(numargs) => Node::Numblock(Numblock {
                        call: method_call,
                        numargs,
                        body: block_body.unwrap_or_else(|| {
                            Box::new(Node::Nil(Nil {
                                expression_l: Loc { start: 0, end: 0 },
                            }))
                        }),
                        begin_l,
                        end_l,
                        expression_l,
                    }),
                };
                return Box::new(result);
            }
            _ => {}
        }

        let method_call = method_call;
        let result = match *method_call {
            Node::Return(Return {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Return(Return {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            Node::Next(Next {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Next(Next {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            Node::Break(Break {
                args,
                keyword_l,
                expression_l,
            }) => {
                let (args, expression_l) =
                    rewrite_args_and_loc(args, expression_l, block_args, block_body);
                Node::Break(Break {
                    args,
                    keyword_l,
                    expression_l,
                })
            }
            other => {
                unreachable!("unsupported method call {:?}", other)
            }
        };

        Box::new(result)
    }
    pub(crate) fn block_pass(amper_t: Token, value: Option<Box<Node>>) -> Box<Node> {
        let amper_l = amper_t.loc;
        let expression_l = amper_l.maybe_join(&value.as_ref().map(|node| *node.expression()));

        Box::new(Node::BlockPass(BlockPass {
            value,
            operator_l: amper_l,
            expression_l,
        }))
    }

    pub(crate) fn attr_asgn(
        receiver: Box<Node>,
        dot_t: Token,
        selector_t: Token,
        buffer: &Buffer,
    ) -> Box<Node> {
        let dot_l = dot_t.loc;
        let selector_l = selector_t.loc;
        let expression_l = receiver.expression().join(&selector_l);
        let receiver: Box<Node> = receiver;

        let mut method_name = string_value(selector_l, buffer);
        method_name.push(b'=');

        match call_type_for_dot(&Some(dot_t)) {
            MethodCallType::Send => Box::new(Node::Send(Send {
                recv: Some(receiver),
                method_name,
                args: vec![],
                dot_l: Some(dot_l),
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })),

            MethodCallType::CSend => Box::new(Node::CSend(CSend {
                recv: receiver,
                method_name,
                args: vec![],
                dot_l,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            })),
        }
    }
    pub(crate) fn index(
        recv: Box<Node>,
        lbrack_t: Token,
        indexes: Vec<Node>,
        rbrack_t: Token,
    ) -> Box<Node> {
        let begin_l = lbrack_t.loc;
        let end_l = rbrack_t.loc;
        let expression_l = recv.expression().join(&end_l);

        // self.rewrite_hash_args_to_kwargs(&mut indexes);

        Box::new(Node::Index(Index {
            recv,
            indexes,
            begin_l,
            end_l,
            expression_l,
        }))
    }
    pub(crate) fn index_asgn(
        recv: Box<Node>,
        lbrack_t: Token,
        indexes: Vec<Node>,
        rbrack_t: Token,
    ) -> Box<Node> {
        let begin_l = lbrack_t.loc;
        let end_l = rbrack_t.loc;
        let expression_l = recv.expression().join(&end_l);

        Box::new(Node::IndexAsgn(IndexAsgn {
            recv,
            indexes,
            value: None,
            begin_l,
            end_l,
            operator_l: None,
            expression_l,
        }))
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

    pub(crate) fn unary_op(op_t: Token, receiver: Box<Node>, buffer: &Buffer) -> Box<Node> {
        // self.value_expr(&receiver)?;

        let selector_l = op_t.loc;
        let expression_l = receiver.expression().join(&selector_l);

        let mut method_name = string_value(selector_l, buffer);
        if method_name.as_bytes() == b"+" || method_name.as_bytes() == b"-" {
            method_name.push(b'@')
        };
        Box::new(Node::Send(Send {
            recv: Some(receiver),
            method_name,
            args: vec![],
            dot_l: None,
            selector_l: Some(selector_l),
            begin_l: None,
            end_l: None,
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn not_op(
        not_t: Token,
        begin_t: Option<Token>,
        receiver: Option<Box<Node>>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        if let Some(receiver) = receiver {
            let receiver = receiver;
            // self.value_expr(&receiver)?;

            let begin_l = not_t.loc;
            let end_l = maybe_loc(&end_t).unwrap_or_else(|| *receiver.expression());

            let expression_l = begin_l.join(&end_l);

            let selector_l = not_t.loc;
            let begin_l = maybe_loc(&begin_t);
            let end_l = maybe_loc(&end_t);

            // let receiver = self.check_condition(receiver)
            Box::new(Node::Send(Send {
                recv: Some(receiver),
                method_name: StringContent::from("!"),
                args: vec![],
                dot_l: None,
                selector_l: Some(selector_l),
                begin_l,
                end_l,
                operator_l: None,
                expression_l,
            }))
        } else {
            let (begin_l, end_l, expression_l) = collection_map(&begin_t, &[], &end_t);

            let nil_node = Box::new(Node::Begin(Begin {
                statements: vec![],
                begin_l,
                end_l,
                expression_l,
            }));

            let selector_l = not_t.loc;
            let expression_l = nil_node.expression().join(&selector_l);
            Box::new(Node::Send(Send {
                recv: Some(nil_node),
                method_name: StringContent::from("!"),
                args: vec![],
                dot_l: None,
                selector_l: Some(selector_l),
                begin_l: None,
                end_l: None,
                operator_l: None,
                expression_l,
            }))
        }
    }
}

enum MethodCallType {
    Send,
    CSend,
}

fn call_type_for_dot(dot_t: &Option<Token>) -> MethodCallType {
    match dot_t.as_ref() {
        Some(token) if token.kind == TokenKind::tANDDOT => MethodCallType::CSend,
        _ => MethodCallType::Send,
    }
}
