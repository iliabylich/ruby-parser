use crate::{
    buffer::Buffer,
    builder::{helpers::string_value, Builder},
    nodes::{BackRef, Cbase, Const, Cvar, Gvar, Ivar, Lvar, NthRef, Self_},
    string_content::StringContent,
    token::Token,
    Node,
};

impl Builder {
    pub(crate) fn self_(self_t: Token) -> Box<Node> {
        let loc = self_t.loc;
        Box::new(Node::Self_(Self_ { expression_l: loc }))
    }
    pub(crate) fn lvar(lvar_t: Token, buffer: &Buffer) -> Box<Node> {
        let loc = lvar_t.loc;
        Box::new(Node::Lvar(Lvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn ivar(ivar_t: Token, buffer: &Buffer) -> Box<Node> {
        let loc = ivar_t.loc;
        Box::new(Node::Ivar(Ivar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn gvar(gvar_t: Token, buffer: &Buffer) -> Box<Node> {
        let loc = gvar_t.loc;
        Box::new(Node::Gvar(Gvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn cvar(cvar_t: Token, buffer: &Buffer) -> Box<Node> {
        let loc = cvar_t.loc;
        Box::new(Node::Cvar(Cvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn back_ref(back_ref_t: Token, buffer: &Buffer) -> Box<Node> {
        let loc = back_ref_t.loc;
        Box::new(Node::BackRef(BackRef {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn nth_ref(nth_ref_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = nth_ref_t.loc;
        let name = string_value(expression_l, buffer).to_string_lossy();
        let name = &name[1..];
        let parsed = name.parse::<usize>();
        let name = StringContent::from(name.as_bytes().to_vec());

        const MAX_NTH_REF: usize = 0b111111111111111111111111111111;
        match parsed {
            Ok(n) if n <= MAX_NTH_REF => {
                // ok
            }
            _ => {
                // TODO: warn
                // DiagnosticMessage::NthRefIsTooBig {
                //      nth_ref: name.clone(),
                // },
            }
        }

        Box::new(Node::NthRef(NthRef { name, expression_l }))
    }

    pub(crate) fn accessible(node: Box<Node>) -> Box<Node> {
        if matches!(&*node, Node::Lvar(_)) {
            match *node {
                Node::Lvar(Lvar { name, expression_l }) => {
                    let name_s = name.as_str();

                    if name_s.ends_with('?') || name_s.ends_with('!') {
                        // TODO: report InvalidIdToGet
                    }

                    // Numbered parameters are not declared anywhere,
                    // so they take precedence over method calls in numblock contexts
                    // TODO: code below must be uncommented and adapted
                    // if try_declare_numparam(name_s, &expression_l) {
                    //     return Box::new(Node::Lvar(Lvar { name, expression_l }));
                    // }

                    // if !self.static_env.is_declared(name_s) {
                    //     return Box::new(Node::Send(Send {
                    //         recv: None,
                    //         method_name: name,
                    //         args: vec![],
                    //         dot_l: None,
                    //         selector_l: Some(expression_l),
                    //         begin_l: None,
                    //         end_l: None,
                    //         operator_l: None,
                    //         expression_l,
                    //     }));
                    // }

                    // if let Some(current_arg) = self.current_arg_stack.top() {
                    //     if current_arg == name_s {
                    //         self.error(
                    //             DiagnosticMessage::CircularArgumentReference {
                    //                 arg_name: name.clone(),
                    //             },
                    //             &expression_l,
                    //         );
                    //     }
                    // }

                    Box::new(Node::Lvar(Lvar { name, expression_l }))
                }
                _ => unreachable!(),
            }
        } else {
            node
        }
    }

    pub(crate) fn const_(const_t: Token, buffer: &Buffer) -> Box<Node> {
        let name_l = const_t.loc;
        let expression_l = name_l;

        Box::new(Node::Const(Const {
            scope: None,
            name: string_value(name_l, buffer),
            double_colon_l: None,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn const_global(colon2_t: Token, name_t: Token, buffer: &Buffer) -> Box<Node> {
        let scope = Box::new(Node::Cbase(Cbase {
            expression_l: colon2_t.loc,
        }));

        let name_l = name_t.loc;
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = colon2_t.loc;

        Box::new(Node::Const(Const {
            scope: Some(scope),
            name: string_value(name_l, buffer),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn const_fetch(
        scope: Box<Node>,
        colon2_t: Token,
        name_t: Token,
        buffer: &Buffer,
    ) -> Box<Node> {
        let scope: Box<Node> = scope;
        let name_l = name_t.loc;
        let expression_l = scope.expression().join(&name_l);
        let double_colon_l = colon2_t.loc;

        Box::new(Node::Const(Const {
            scope: Some(scope),
            name: string_value(name_l, buffer),
            double_colon_l: Some(double_colon_l),
            name_l,
            expression_l,
        }))
    }
}
