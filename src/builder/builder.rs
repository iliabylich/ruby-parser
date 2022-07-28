use crate::{
    builder::{CString, Constructor, RustConstructor},
    lexer::buffer::Buffer,
    loc::{loc, Loc},
    nodes::*,
    string_content::StringContent,
    token::{Token, TokenKind},
    Node,
};

pub(crate) struct Builder<C: Constructor = RustConstructor> {
    phantom: std::marker::PhantomData<C>,
}

fn node_ptr_to_box(ptr: *mut std::ffi::c_void) -> Box<Node> {
    unsafe { Box::from_raw(ptr as *mut Node) }
}

fn cstring_value(loc: Loc, buffer: &Buffer) -> CString {
    CString::from(buffer.slice(loc.start, loc.end).unwrap())
}
fn string_value(loc: Loc, buffer: &Buffer) -> StringContent {
    StringContent::from(buffer.slice(loc.start, loc.end).unwrap())
}

impl<C: Constructor> Builder<C> {
    // Singletons
    pub(crate) fn nil(nil_t: Token) -> Box<Node> {
        let loc = nil_t.loc;
        Box::new(Node::Nil(Nil { expression_l: loc }))
    }
    pub(crate) fn true_(true_t: Token) -> Box<Node> {
        let loc = true_t.loc;
        Box::new(Node::True(True { expression_l: loc }))
    }
    pub(crate) fn false_(false_t: Token) -> Box<Node> {
        let loc = false_t.loc;
        Box::new(Node::False(False { expression_l: loc }))
    }

    // Numerics
    pub(crate) fn integer(integer_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = integer_t.loc;
        Box::new(Node::Int(Int {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }
    pub(crate) fn float(float_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = float_t.loc;
        Box::new(Node::Float(Float {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }
    pub(crate) fn rational(rational_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = rational_t.loc;
        Box::new(Node::Rational(Rational {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }
    pub(crate) fn complex(complex_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = complex_t.loc;
        Box::new(Node::Complex(Complex {
            value: string_value(expression_l, buffer),
            operator_l: None,
            expression_l,
        }))
    }

    pub(crate) fn unary_num(unary_t: Token, mut numeric: Box<Node>, buffer: &Buffer) -> Box<Node> {
        let new_operator_l = unary_t.loc;

        match &mut *numeric {
            Node::Int(Int {
                value,
                expression_l,
                operator_l,
            })
            | Node::Float(Float {
                value,
                expression_l,
                operator_l,
            })
            | Node::Rational(Rational {
                value,
                operator_l,
                expression_l,
            })
            | Node::Complex(Complex {
                value,
                operator_l,
                expression_l,
            }) => {
                *operator_l = Some(new_operator_l);
                *expression_l = new_operator_l.join(expression_l);
                *value = string_value(*expression_l, buffer);
            }

            _ => {}
        }

        numeric
    }

    // Special constants
    pub(crate) fn __line__(line_t: Token) -> Box<Node> {
        let loc = line_t.loc;
        Box::new(Node::Line(Line { expression_l: loc }))
    }
    pub(crate) fn __file__(file_t: Token) -> Box<Node> {
        let loc = file_t.loc;
        Box::new(Node::File(File { expression_l: loc }))
    }
    pub(crate) fn __encoding__(encoding_t: Token) -> Box<Node> {
        let loc = encoding_t.loc;
        Box::new(Node::Encoding(Encoding { expression_l: loc }))
    }

    // Strings

    pub(crate) fn str_node(
        begin_t: Option<Token>,
        value: StringContent,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        if let Some(Token {
            kind: TokenKind::tHEREDOC_BEG,
            ..
        }) = &begin_t
        {
            let (heredoc_body_l, heredoc_end_l, expression_l) =
                heredoc_map(&begin_t, &parts, &end_t);

            Box::new(Node::Heredoc(Heredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let (begin_l, end_l, expression_l) = collection_map(&begin_t, &parts, &end_t);

            Box::new(Node::Str(Str {
                value,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    pub(crate) fn string_internal(string_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = string_t.loc;
        let value = string_value(expression_l, buffer);
        Box::new(Node::Str(Str {
            value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn string_compose(
        begin_t: Option<Token>,
        parts: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        match &parts[..] {
            [] => {
                return Self::str_node(begin_t, StringContent::from(""), parts, end_t);
            }

            [Node::Str(_) | Node::Dstr(_) | Node::Heredoc(_)]
                if begin_t.is_none() && end_t.is_none() =>
            {
                return Box::new(parts.into_iter().next().expect("expected at least 1 item"));
            }

            [Node::Str(Str { value, .. })] => {
                return Self::str_node(begin_t, value.clone(), parts, end_t);
            }

            [Node::Dstr(_) | Node::Heredoc(_)] => {
                unreachable!("dstr or heredoc string without begin_t/end_t")
            }

            _ => {}
        }

        todo!()
    }

    pub(crate) fn character(char_t: Token) -> Box<Node> {
        let expression_l = char_t.loc;
        let begin_l = loc!(expression_l.start, expression_l.start + 1);

        let value = if let Token {
            kind: TokenKind::tCHAR,
            value: Some(value),
            ..
        } = char_t
        {
            value
        } else {
            unreachable!()
        };

        let value = StringContent::from(value);
        Box::new(Node::Str(Str {
            value,
            begin_l: Some(begin_l),
            end_l: None,
            expression_l,
        }))
    }

    // Symbols
    pub(crate) fn symbol(start_t: Token, value_t: Token, buffer: &Buffer) -> Box<Node> {
        let begin_l = start_t.loc;
        let value_l = value_t.loc;
        let expression_l = begin_l.join(&value_l);
        let value = string_value(value_l, buffer);
        // TODO: validate_sym_value
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: Some(begin_l),
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_internal(symbol_t: Token, buffer: &Buffer) -> Box<Node> {
        let expression_l = symbol_t.loc;
        let value = string_value(expression_l, buffer);
        // TODO: validate_sym_value
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_compose(begin_t: Token, parts: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);

        if parts.len() == 1 && matches!(&parts[0], Node::Str(_)) {
            let part = parts.into_iter().next().unwrap();
            let value = if let Node::Str(Str { value, .. }) = part {
                value
            } else {
                unreachable!()
            };

            // TODO: validate_sym_value

            return Box::new(Node::Sym(Sym {
                name: value,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }));
        }

        Box::new(Node::Dsym(Dsym {
            parts,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    // Executable string

    pub(crate) fn xstring_compose(begin_t: Token, parts: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;

        if begin_t.is(TokenKind::tXHEREDOC_BEG) {
            let heredoc_body_l = collection_expr(&parts).unwrap_or(end_l);
            let heredoc_end_l = end_l;
            let expression_l = begin_l;

            Box::new(Node::XHeredoc(XHeredoc {
                parts,
                heredoc_body_l,
                heredoc_end_l,
                expression_l,
            }))
        } else {
            let expression_l = begin_l.join(&end_l);

            Box::new(Node::Xstr(Xstr {
                parts,
                begin_l,
                end_l,
                expression_l,
            }))
        }
    }

    // Regular expressions

    pub(crate) fn regexp_options(regexp_end_t: &Token, buffer: &Buffer) -> Option<Box<Node>> {
        let expression_l = regexp_end_t.loc;

        if expression_l.size() == 1 {
            // no regexp options, only trailing "/"
            return None;
        }

        // exclude leading '/'
        let expression_l = expression_l.adjust_start(1);
        let options = string_value(expression_l, buffer);

        let mut options = options.as_str().chars().collect::<Vec<_>>();
        options.sort_unstable();
        options.dedup();
        let options = if options.is_empty() {
            None
        } else {
            Some(StringContent::from(options.into_iter().collect::<String>()))
        };

        Some(Box::new(Node::RegOpt(RegOpt {
            options,
            expression_l,
        })))
    }

    pub(crate) fn regexp_compose(
        begin_t: Token,
        parts: Vec<Node>,
        end_t: Token,
        options: Option<Box<Node>>,
    ) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc.resize(1);
        let expression_l = begin_l
            .join(&end_l)
            .maybe_join(&options.as_ref().map(|options| *options.expression()));

        match options.as_deref() {
            Some(Node::RegOpt(RegOpt {
                options,
                expression_l,
            })) => {
                // TODO: validate_static_regexp
            }
            None => {
                // TODO: validate_static_regexp
            }
            _ => unreachable!("bug: must be Option<RegOpt>"),
        }

        Box::new(Node::Regexp(Regexp {
            parts,
            options,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Arrays

    pub(crate) fn array(
        begin_t: Option<Token>,
        elements: Vec<Node>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!("builder.array")
    }

    pub(crate) fn splat(star_t: Token, value: Box<Node>) -> Box<Node> {
        let operator_l = star_t.loc;
        let expression_l = operator_l.join(value.expression());
        Box::new(Node::Splat(Splat {
            value: Some(value),
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn nameless_splat(star_t: Token) -> Box<Node> {
        let operator_l = star_t.loc;
        let expression_l = operator_l;
        Box::new(Node::Splat(Splat {
            value: None,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn word(parts: Vec<Node>) -> Box<Node> {
        debug_assert!(!parts.is_empty());

        if parts.len() == 1 && matches!(&parts[0], Node::Str(_) | Node::Dstr(_)) {
            let part = parts.into_iter().next().expect("expected 1 element");
            return Box::new(part);
        }

        let (begin_l, end_l, expression_l) = collection_map(&None, &parts, &None);
        Box::new(Node::Dstr(Dstr {
            parts,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    pub(crate) fn words_compose(begin_t: Token, elements: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn symbols_compose(begin_t: Token, elements: Vec<Node>, end_t: Token) -> Box<Node> {
        let elements = elements
            .into_iter()
            .map(|part| match part {
                Node::Str(Str {
                    value,
                    begin_l,
                    end_l,
                    expression_l,
                }) => {
                    // TODO: validate_sym_value
                    Node::Sym(Sym {
                        name: value,
                        begin_l,
                        end_l,
                        expression_l,
                    })
                }
                Node::Dstr(Dstr {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }) => Node::Dsym(Dsym {
                    parts,
                    begin_l,
                    end_l,
                    expression_l,
                }),
                other => other,
            })
            .collect::<Vec<_>>();

        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        let expression_l = begin_l.join(&end_l);
        Box::new(Node::Array(Array {
            elements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    // Hashes

    // Ranges

    // Access
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
        let name = cstring_value(loc, buffer);
        node_ptr_to_box(C::gvar_node(name, loc))
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
        let name = cstring_value(loc, buffer);
        node_ptr_to_box(C::back_ref_node(name, loc))
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
        todo!("builder.accessible")
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

    pub(crate) fn const_global(colon2_t: Token, name_t: Token) -> Box<Node> {
        todo!("builder.const_global")
    }

    // Assignments
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
        todo!("builder.const_op_assignable")
    }

    pub(crate) fn assign(lhs: Box<Node>, op_t: Token, rhs: Box<Node>) -> Box<Node> {
        todo!("builder.assign")
    }

    pub(crate) fn op_assign(lhs: Box<Node>, op_t: Token, rhs: Box<Node>) -> Box<Node> {
        todo!("builder.op_assign")
    }

    // Class and module definition

    // Method (un)definition

    pub(crate) fn def_method() -> Box<Node> {
        todo!("builder.def_method")
    }

    pub(crate) fn def_endless_method() -> Box<Node> {
        todo!("builder.def_endless_method")
    }

    pub(crate) fn def_singleton() -> Box<Node> {
        todo!("builder.def_singleton")
    }

    pub(crate) fn def_endless_singleton() -> Box<Node> {
        todo!("builder.def_endless_singleton")
    }

    pub(crate) fn undef(undef_t: Token, names: Vec<Node>) -> Box<Node> {
        debug_assert!(!names.is_empty());

        let keyword_l = undef_t.loc;
        let expression_l = keyword_l.join(names.last().unwrap().expression());
        Box::new(Node::Undef(Undef {
            names,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn alias(alias_t: Token, to: Box<Node>, from: Box<Node>) -> Box<Node> {
        let keyword_l = alias_t.loc;
        let expression_l = keyword_l.join(from.expression());
        Box::new(Node::Alias(Alias {
            to,
            from,
            keyword_l,
            expression_l,
        }))
    }

    // Formatl arguments

    // Method calls

    pub(crate) fn forwarded_args() {}
    pub(crate) fn call_method() {}
    pub(crate) fn call_lambda() {}
    pub(crate) fn block() {}
    pub(crate) fn block_pass() {}
    pub(crate) fn attr_asgn() {}
    pub(crate) fn index() {}
    pub(crate) fn index_asgn() {}
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
    pub(crate) fn unary_op() {}
    pub(crate) fn not_op(
        not_t: Token,
        begin_t: Option<Token>,
        receiver: Option<Box<Node>>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!()
    }

    // Logical operations: and, or

    pub(crate) fn logical_op(lhs: Box<Node>, op_t: Token, rhs: Box<Node>) -> Box<Node> {
        // TODO: value_expr(lhs)

        let operator_l = op_t.loc;
        let expression_l = lhs.expression().join(rhs.expression());

        match operator_l.size() {
            2 => {
                // kOR
                Box::new(Node::And(And {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            3 => {
                // kAND
                Box::new(Node::And(And {
                    lhs,
                    rhs,
                    operator_l,
                    expression_l,
                }))
            }
            _ => unreachable!("only kOR (size = 2) or kAND(size = 3) is supported"),
        }
    }

    // Conditionals
    pub(crate) fn condition(
        cond_t: Token,
        cond: Box<Node>,
        then_t: Token,
        if_true: Option<Box<Node>>,
        else_t: Option<Token>,
        if_false: Option<Box<Node>>,
        end_t: Option<Token>,
    ) -> Box<Node> {
        todo!("condition")
    }

    pub(crate) fn condition_mod(
        if_true: Option<Box<Node>>,
        if_false: Option<Box<Node>>,
        cond_t: Token,
        cond: Box<Node>,
    ) -> Box<Node> {
        todo!("condition_mod")
    }

    pub(crate) fn ternary(
        cond: Box<Node>,
        question_t: Token,
        if_true: Box<Node>,
        colon_t: Token,
        if_false: Box<Node>,
    ) -> Box<Node> {
        todo!("ternary")
    }

    // Case matching

    // Loops

    // Keywords

    // BEGIN, END
    pub(crate) fn preexe(
        preexe_t: Token,
        lcurly_t: Token,
        body: Option<Box<Node>>,
        rcurly_t: Token,
    ) -> Box<Node> {
        let keyword_l = preexe_t.loc;
        let begin_l = lcurly_t.loc;
        let end_l = rcurly_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Preexe(Preexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }
    pub(crate) fn postexe(
        postexe_t: Token,
        lcurly_t: Token,
        body: Option<Box<Node>>,
        rcurly_t: Token,
    ) -> Box<Node> {
        let keyword_l = postexe_t.loc;
        let begin_l = lcurly_t.loc;
        let end_l = rcurly_t.loc;
        let expression_l = keyword_l.join(&end_l);

        Box::new(Node::Postexe(Postexe {
            body,
            keyword_l,
            begin_l,
            end_l,
            expression_l,
        }))
    }

    // Exception handling

    pub(crate) fn rescue_body(
        rescue_t: Token,
        exc_list: Vec<Node>,
        exc_var: Option<(Token, Box<Node>)>,
        then_t: Option<Token>,
        body: Option<Box<Node>>,
    ) -> Box<Node> {
        let exc_list = Self::array(None, exc_list, None);
        todo!("builder.rescue_body")
    }

    pub(crate) fn begin_body(
        compound_stmt: Option<Box<Node>>,
        rescue_bodies: Vec<Node>,
        opt_else: Option<(Token, Option<Box<Node>>)>,
        opt_ensure: Option<(Token, Option<Box<Node>>)>,
    ) -> Box<Node> {
        todo!("builder.begin_body")
    }

    // Expression grouping
    pub(crate) fn compstmt(statements: Vec<Node>) -> Box<Node> {
        debug_assert!(!statements.is_empty());

        if statements.len() == 1 {
            Box::new(statements.into_iter().next().unwrap())
        } else {
            let (begin_l, end_l, expression_l) = nodes_locs(&statements);

            Box::new(Node::Begin(Begin {
                statements,
                begin_l: Some(begin_l),
                end_l: Some(end_l),
                expression_l,
            }))
        }
    }

    pub(crate) fn begin(begin_t: Token, statements: Vec<Node>, end_t: Token) -> Box<Node> {
        let begin_l = begin_t.loc;
        let end_l = end_t.loc;
        Box::new(Node::Begin(Begin {
            statements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l: begin_l.join(&end_l),
        }))
    }

    pub(crate) fn group(nodes: Vec<Node>) -> Box<Node> {
        debug_assert!(nodes.len() > 0);

        if nodes.len() == 1 {
            return Box::new(nodes.into_iter().next().unwrap());
        }

        let (begin_l, end_l, expression_l) = nodes_locs(&nodes);

        Box::new(Node::Begin(Begin {
            statements: nodes,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l,
        }))
    }

    pub(crate) fn group_with_trailing_comma(nodes: Vec<Node>, trailing_comma: Token) -> Box<Node> {
        todo!("builder.group_with_trailing_comma")
    }

    // Pattern matching

    pub(crate) fn case_match() {}

    pub(crate) fn match_pattern(value: Box<Node>, assoc_t: Token, pattern: Box<Node>) -> Box<Node> {
        let operator_l = assoc_t.loc;
        let expression_l = value.expression().join(pattern.expression());

        Box::new(Node::MatchPattern(MatchPattern {
            value,
            pattern,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn match_pattern_p(value: Box<Node>, in_t: Token, pattern: Box<Node>) -> Box<Node> {
        let operator_l = in_t.loc;
        let expression_l = value.expression().join(pattern.expression());

        Box::new(Node::MatchPatternP(MatchPatternP {
            value,
            pattern,
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn in_pattern() {}
    pub(crate) fn if_guard() {}
    pub(crate) fn unless_guard() {}
    pub(crate) fn match_var() {}
    pub(crate) fn match_hash_var() {}
    pub(crate) fn match_hash_var_from_str() {}
    pub(crate) fn match_rest() {}
    pub(crate) fn hash_pattern() {}
    pub(crate) fn array_pattern() {}
    pub(crate) fn find_pattern() {}
    pub(crate) fn const_pattern() {}
    pub(crate) fn pin() {}
    pub(crate) fn match_alt() {}
    pub(crate) fn match_as() {}
    pub(crate) fn match_nil_pattern() {}
    pub(crate) fn match_pair() {}
    pub(crate) fn match_label() {}
}

// Loc helpers

fn nodes_locs(nodes: &[Node]) -> (Loc, Loc, Loc) {
    debug_assert!(nodes.len() > 0);

    let begin = nodes.first().unwrap().expression().start;
    let end = nodes.last().unwrap().expression().end;

    let begin_l = loc!(begin, begin + 1);
    let end_l = loc!(end, end + 1);
    let expression_l = begin_l.join(&end_l);

    (begin_l, end_l, expression_l)
}

fn collection_map(
    begin_t: &Option<Token>,
    nodes: &[Node],
    end_t: &Option<Token>,
) -> (Option<Loc>, Option<Loc>, Loc) {
    let begin_l = begin_t.as_ref().map(|tok| tok.loc);
    let end_l = end_t.as_ref().map(|tok| tok.loc);

    let expression_l = collection_expr(nodes);
    let expression_l = join_maybe_locs(&begin_l, &expression_l);
    let expression_l = join_maybe_locs(&expression_l, &end_l);
    let expression_l = expression_l.unwrap_or_else(|| {
        unreachable!("empty collection without begin_t/end_t, can't build source map");
    });

    (begin_l, end_l, expression_l)
}

fn collection_expr(nodes: &[Node]) -> Option<Loc> {
    let lhs = nodes.first().map(|node| *node.expression());
    let rhs = nodes.last().map(|node| *node.expression());
    join_maybe_locs(&lhs, &rhs)
}

fn join_maybe_locs(lhs: &Option<Loc>, rhs: &Option<Loc>) -> Option<Loc> {
    match (lhs, rhs) {
        (None, None) => None,
        (None, Some(rhs)) => Some(*rhs),
        (Some(lhs), None) => Some(*lhs),
        (Some(lhs), Some(rhs)) => Some(lhs.join(rhs)),
    }
}

fn heredoc_map(begin_t: &Option<Token>, nodes: &[Node], end_t: &Option<Token>) -> (Loc, Loc, Loc) {
    todo!("builder.heredoc_map")
}

// Regexp heleprs
fn static_regexp_captures(node: &Node) -> Option<Vec<String>> {
    None
}
