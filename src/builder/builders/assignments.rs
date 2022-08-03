use crate::{
    buffer::Buffer,
    builder::{builders::helpers::string_value, Builder},
    nodes::{
        AndAsgn, BackRef, CSend, Casgn, Const, Cvar, Cvasgn, Encoding, False, File, Gvar, Gvasgn,
        Index, IndexAsgn, Ivar, Ivasgn, Line, Lvar, Lvasgn, MatchVar, Nil, NthRef, OpAsgn, OrAsgn,
        Self_, Send, True,
    },
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn assignable(node: Box<Node>) -> Box<Node> {
        let node = match *node {
            Node::Cvar(Cvar { name, expression_l }) => Node::Cvasgn(Cvasgn {
                name,
                value: None,
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Ivar(Ivar { name, expression_l }) => Node::Ivasgn(Ivasgn {
                name,
                value: None,
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Gvar(Gvar { name, expression_l }) => Node::Gvasgn(Gvasgn {
                name,
                value: None,
                name_l: expression_l,
                operator_l: None,
                expression_l,
            }),
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => {
                // TODO: check dynamic constant assignment
                Node::Casgn(Casgn {
                    scope,
                    name,
                    value: None,
                    double_colon_l,
                    name_l,
                    operator_l: None,
                    expression_l,
                })
            }
            Node::Lvar(Lvar { name, expression_l }) => {
                // TODO: check assignment to numparam
                // TODO: check if name is reserved for numparam

                // TODO: save `name` as local variable

                Node::Lvasgn(Lvasgn {
                    name,
                    value: None,
                    name_l: expression_l,
                    operator_l: None,
                    expression_l,
                })
            }
            Node::MatchVar(MatchVar {
                name,
                name_l,
                expression_l,
            }) => {
                // TODO: check assignment to numparam
                // TODO: check if name is reserved for numparam

                Node::MatchVar(MatchVar {
                    name,
                    name_l,
                    expression_l,
                })
            }
            node @ Node::Self_(Self_ { .. }) => {
                // TODO: report assignment to `self`
                node
            }
            node @ Node::Nil(Nil { .. }) => {
                // TODO: report assignment to `nil`
                node
            }
            node @ Node::True(True { .. }) => {
                // TODO: report assignment to `true`
                node
            }
            node @ Node::False(False { .. }) => {
                // TODO: report assignment to `false`
                node
            }
            node @ Node::File(File { .. }) => {
                // TODO: report assignment to `__FILE__`
                node
            }
            node @ Node::Line(Line { .. }) => {
                // TODO: report assignment to `__LINE__`
                node
            }
            node @ Node::Encoding(Encoding { .. }) => {
                // TODO: report assignment to `__ENCODING__`
                node
            }
            node @ Node::BackRef(BackRef { .. }) => {
                // TODO: report assignment to back ref
                node
            }
            node @ Node::NthRef(NthRef { .. }) => {
                // TODO: report assignment to nth ref
                node
            }
            other => unreachable!("{:?} can't be used in assignment", other),
        };

        Box::new(node)
    }

    pub(crate) fn const_op_assignable(node: Box<Node>) -> Box<Node> {
        match *node {
            Node::Const(Const {
                scope,
                name,
                double_colon_l,
                name_l,
                expression_l,
            }) => Box::new(Node::Casgn(Casgn {
                scope,
                name,
                value: None,
                double_colon_l,
                name_l,
                operator_l: None,
                expression_l,
            })),
            other => {
                unreachable!("unsupported const_op_assignable arument: {:?}", other)
            }
        }
    }

    pub(crate) fn assign(mut lhs: Box<Node>, eql_t: Token, rhs: Box<Node>) -> Box<Node> {
        let op_l = Some(eql_t.loc);
        let expr_l = lhs.expression().join(rhs.expression());

        match &mut *lhs {
            Node::Cvasgn(Cvasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Ivasgn(Ivasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Gvasgn(Gvasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Lvasgn(Lvasgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::Casgn(Casgn {
                expression_l,
                operator_l,
                value,
                ..
            })
            | Node::IndexAsgn(IndexAsgn {
                expression_l,
                operator_l,
                value,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                *value = Some(rhs);
            }
            Node::Send(Send {
                expression_l,
                operator_l,
                args,
                ..
            })
            | Node::CSend(CSend {
                expression_l,
                operator_l,
                args,
                ..
            }) => {
                *expression_l = expr_l;
                *operator_l = op_l;
                if args.is_empty() {
                    *args = vec![*rhs];
                } else {
                    unreachable!("can't assign to method call with args")
                }
            }
            other => unreachable!("{:?} can't be used in assignment", other),
        }

        lhs
    }

    pub(crate) fn op_assign(
        mut lhs: Box<Node>,
        op_t: Token,
        rhs: Box<Node>,
        buffer: &Buffer,
    ) -> Box<Node> {
        let operator_l = op_t.loc;
        let mut operator = string_value(operator_l, buffer);
        operator.pop();
        let expression_l = lhs.expression().join(rhs.expression());

        match &*lhs {
            Node::Gvasgn(_)
            | Node::Ivasgn(_)
            | Node::Lvasgn(_)
            | Node::Cvasgn(_)
            | Node::Casgn(_)
            | Node::Send(_)
            | Node::CSend(_) => {
                // ignore
            }
            Node::Index(_) => match *lhs {
                Node::Index(Index {
                    recv,
                    indexes,
                    begin_l,
                    end_l,
                    expression_l,
                }) => {
                    lhs = Box::new(Node::IndexAsgn(IndexAsgn {
                        recv,
                        indexes,
                        value: None,
                        begin_l,
                        end_l,
                        operator_l: None,
                        expression_l,
                    }));
                }
                _ => unreachable!(),
            },
            Node::BackRef(BackRef { name, expression_l }) => {
                // TODO: report CantSetVariable
                // self.error(
                //     DiagnosticMessage::CantSetVariable {
                //         var_name: name.clone(),
                //     },
                //     expression_l,
                // );

                // and ignore
            }
            Node::NthRef(NthRef { name, expression_l }) => {
                // TODO: report CantSetVariable
                // self.error(
                //     DiagnosticMessage::CantSetVariable {
                //         var_name: format!("${}", name),
                //     },
                //     expression_l,
                // );

                // and ignore
            }
            _ => unreachable!("unsupported op_assign lhs {:?}", lhs),
        }

        let recv: Box<Node> = lhs;
        let value: Box<Node> = rhs;

        let result = match operator.as_bytes() {
            b"&&" => Node::AndAsgn(AndAsgn {
                recv,
                value,
                operator_l,
                expression_l,
            }),
            b"||" => Node::OrAsgn(OrAsgn {
                recv,
                value,
                operator_l,
                expression_l,
            }),
            _ => Node::OpAsgn(OpAsgn {
                recv,
                operator,
                value,
                operator_l,
                expression_l,
            }),
        };

        Box::new(result)
    }
}
