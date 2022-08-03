use crate::{
    buffer::Buffer,
    builder::{
        helpers::{collection_map, maybe_loc, maybe_string_value, string_value},
        Builder,
    },
    nodes::{
        Arg, Args, Blockarg, ForwardArg, ForwardedArgs, Kwarg, Kwnilarg, Kwoptarg, Kwrestarg, Mlhs,
        Optarg, Procarg0, Restarg, Shadowarg,
    },
    Node, Token,
};

impl Builder {
    pub(crate) fn args(
        begin_t: Option<Token>,
        args: Vec<Node>,
        end_t: Option<Token>,
    ) -> Option<Box<Node>> {
        // self.check_duplicate_args(&args, &mut HashMap::new());
        // self.validate_no_forward_arg_after_restarg(&args);

        if begin_t.is_none() && args.is_empty() && end_t.is_none() {
            return None;
        }

        let (begin_l, end_l, expression_l) = collection_map(&begin_t, &args, &end_t);

        Some(Box::new(Node::Args(Args {
            args,
            expression_l,
            begin_l,
            end_l,
        })))
    }

    pub(crate) fn forwarded_args(dots_t: Token) -> Box<Node> {
        Box::new(Node::ForwardedArgs(ForwardedArgs {
            expression_l: dots_t.loc,
        }))
    }

    pub(crate) fn forward_arg(dots_t: Token) -> Box<Node> {
        Box::new(Node::ForwardArg(ForwardArg {
            expression_l: dots_t.loc,
        }))
    }

    pub(crate) fn arg(name_t: Token, buffer: &Buffer) -> Box<Node> {
        let name_l = name_t.loc;
        let name = string_value(name_l, buffer);

        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Arg(Arg {
            name,
            expression_l: name_l,
        }))
    }

    pub(crate) fn optarg(
        name_t: Token,
        eql_t: Token,
        default: Box<Node>,
        buffer: &Buffer,
    ) -> Box<Node> {
        let operator_l = eql_t.loc;
        let name_l = name_t.loc;
        let expression_l = name_t.loc.join(default.expression());

        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Optarg(Optarg {
            name,
            default,
            name_l,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn restarg(star_t: Token, name_t: Option<Token>, buffer: &Buffer) -> Box<Node> {
        let (name, name_l) = if let Some(name_t) = name_t {
            let name_l = name_t.loc;
            let name = string_value(name_l, buffer);
            // self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
        };

        let operator_l = star_t.loc;
        let expression_l = operator_l.maybe_join(&name_l);

        Box::new(Node::Restarg(Restarg {
            name,
            operator_l,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn kwarg(name_t: Token, buffer: &Buffer) -> Box<Node> {
        let name_l = name_t.loc;
        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let expression_l = name_l;
        let name_l = expression_l.adjust_end(-1);

        Box::new(Node::Kwarg(Kwarg {
            name,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn kwoptarg(name_t: Token, default: Box<Node>, buffer: &Buffer) -> Box<Node> {
        let name_l = name_t.loc;
        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        let label_l = name_l;
        let name_l = label_l.adjust_end(-1);
        let expression_l = default.expression().join(&label_l);

        Box::new(Node::Kwoptarg(Kwoptarg {
            name,
            default,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn kwrestarg(dstar_t: Token, name_t: Option<Token>, buffer: &Buffer) -> Box<Node> {
        let (name, name_l) = if let Some(name_t) = name_t {
            let name_l = name_t.loc;
            let name = string_value(name_l, buffer);
            // self.check_reserved_for_numparam(name.as_str(), &name_l)?;
            (Some(name), Some(name_l))
        } else {
            (None, None)
        };

        let operator_l = dstar_t.loc;
        let expression_l = operator_l.maybe_join(&name_l);

        Box::new(Node::Kwrestarg(Kwrestarg {
            name,
            operator_l,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn kwnilarg(dstar_t: Token, nil_t: Token) -> Box<Node> {
        let dstar_l = dstar_t.loc;
        let nil_l = nil_t.loc;
        let expression_l = dstar_l.join(&nil_l);
        Box::new(Node::Kwnilarg(Kwnilarg {
            name_l: nil_l,
            expression_l,
        }))
    }

    pub(crate) fn shadowarg(name_t: Token, buffer: &Buffer) -> Box<Node> {
        let name_l = name_t.loc;
        let name = string_value(name_l, buffer);
        // self.check_reserved_for_numparam(name.as_str(), &name_l)?;

        Box::new(Node::Shadowarg(Shadowarg {
            name,
            expression_l: name_l,
        }))
    }

    pub(crate) fn blockarg(amper_t: Token, name_t: Option<Token>, buffer: &Buffer) -> Box<Node> {
        let name_l = maybe_loc(&name_t);
        let name = maybe_string_value(name_l, buffer);
        if let (Some(_name_l), Some(_name)) = (name_l.as_ref(), name.as_ref()) {
            // self.check_reserved_for_numparam(name, name_l)?;
        }

        let operator_l = amper_t.loc;
        let expression_l = operator_l.maybe_join(&name_l);

        Box::new(Node::Blockarg(Blockarg {
            name,
            operator_l,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn procarg0(arg: Box<Node>) -> Box<Node> {
        match *arg {
            Node::Mlhs(Mlhs {
                items,
                begin_l,
                end_l,
                expression_l,
            }) => Box::new(Node::Procarg0(Procarg0 {
                args: items,
                begin_l,
                end_l,
                expression_l,
            })),
            Node::Arg(Arg { name, expression_l }) => Box::new(Node::Procarg0(Procarg0 {
                args: vec![Node::Arg(Arg { name, expression_l })],
                begin_l: None,
                end_l: None,
                expression_l,
            })),
            other => {
                unreachable!("unsupported procarg0 child {:?}", other)
            }
        }
    }
}
