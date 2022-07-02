use crate::{
    builder::{CString, Constructor, RustConstructor},
    lexer::buffer::Buffer,
    nodes::*,
    string_content::StringContent,
    Loc, Node, Token,
};

pub(crate) struct Builder<C: Constructor = RustConstructor> {
    phantom: std::marker::PhantomData<C>,
}

macro_rules! node_ptr_to_box {
    ($ptr:expr) => {
        Box::from_raw($ptr as *mut Node<'a>)
    };
}

fn cstring_value(loc: Loc, buffer: &Buffer) -> CString {
    CString::from(buffer.slice(loc.start, loc.end).unwrap())
}
fn string_value<'a>(loc: Loc, buffer: &Buffer<'a>) -> StringContent<'a> {
    StringContent::from(buffer.slice(loc.start, loc.end).unwrap())
}

impl<'a, C: Constructor> Builder<C> {
    // Singletons
    pub(crate) fn nil(nil_t: Token) -> Box<Node> {
        let loc = nil_t.loc();
        Box::new(Node::Nil(Nil { expression_l: loc }))
    }
    pub(crate) fn true_(true_t: Token) -> Box<Node> {
        let loc = true_t.loc();
        Box::new(Node::True(True { expression_l: loc }))
    }
    pub(crate) fn false_(false_t: Token) -> Box<Node> {
        let loc = false_t.loc();
        Box::new(Node::False(False { expression_l: loc }))
    }

    // Numerics
    pub(crate) fn integer(integer_t: Token) -> Box<Node> {
        todo!()
    }
    pub(crate) fn float(float_t: Token) -> Box<Node> {
        todo!()
    }
    pub(crate) fn rational(rational_t: Token) -> Box<Node> {
        todo!()
    }
    pub(crate) fn complex(complex_t: Token) -> Box<Node> {
        todo!()
    }

    // Special constants
    pub(crate) fn __line__(line_t: Token) -> Box<Node> {
        let loc = line_t.loc();
        Box::new(Node::Line(Line { expression_l: loc }))
    }
    pub(crate) fn __file__(file_t: Token) -> Box<Node> {
        let loc = file_t.loc();
        Box::new(Node::File(File { expression_l: loc }))
    }
    pub(crate) fn __encoding__(encoding_t: Token) -> Box<Node> {
        let loc = encoding_t.loc();
        Box::new(Node::Encoding(Encoding { expression_l: loc }))
    }

    // Strings

    // Symbols
    pub(crate) fn symbol(
        start_t: Token<'a>,
        value_t: Token<'a>,
        buffer: &Buffer<'a>,
    ) -> Box<Node<'a>> {
        let begin_l = start_t.loc();
        let value_l = value_t.loc();
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

    pub(crate) fn symbol_internal(symbol_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let expression_l = symbol_t.loc();
        let value = string_value(expression_l, buffer);
        // TODO: validate_sym_value
        Box::new(Node::Sym(Sym {
            name: value,
            begin_l: None,
            end_l: None,
            expression_l,
        }))
    }

    pub(crate) fn symbol_compose(
        begin_t: Token<'a>,
        parts: Vec<Node<'a>>,
        end_t: Token<'a>,
    ) -> Box<Node<'a>> {
        let begin_l = begin_t.loc();
        let end_l = end_t.loc();
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

    // Regular expressions

    // Arrays

    pub(crate) fn array(
        begin_t: Option<Token<'a>>,
        elements: Vec<Node<'a>>,
        end_t: Option<Token<'a>>,
    ) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn splat(star_t: Token<'a>, value: Box<Node<'a>>) -> Box<Node<'a>> {
        let operator_l = star_t.loc();
        let expression_l = operator_l.join(value.expression());
        Box::new(Node::Splat(Splat {
            value: Some(value),
            operator_l,
            expression_l,
        }))
    }

    pub(crate) fn nameless_splat(star_t: Token<'a>) -> Box<Node<'a>> {
        let operator_l = star_t.loc();
        let expression_l = operator_l;
        Box::new(Node::Splat(Splat {
            value: None,
            operator_l,
            expression_l,
        }))
    }

    // Hashes

    // Ranges

    // Access
    pub(crate) fn self_(self_t: Token) -> Box<Node> {
        let loc = self_t.loc();
        Box::new(Node::Self_(Self_ { expression_l: loc }))
    }
    pub(crate) fn lvar(lvar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = lvar_t.loc();
        Box::new(Node::Lvar(Lvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn ivar(ivar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = ivar_t.loc();
        Box::new(Node::Ivar(Ivar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn gvar(gvar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = gvar_t.loc();
        let name = cstring_value(loc, buffer);
        unsafe { node_ptr_to_box!(C::gvar_node(name, loc)) }
    }
    pub(crate) fn cvar(cvar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = cvar_t.loc();
        Box::new(Node::Cvar(Cvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn back_ref(back_ref_t: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        let loc = back_ref_t.loc();
        let name = cstring_value(loc, buffer);
        unsafe { node_ptr_to_box!(C::back_ref_node(name, loc)) }
    }
    pub(crate) fn nth_ref(nth_ref_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let expression_l = nth_ref_t.loc();
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

    pub(crate) fn accessible(node: Box<Node<'a>>) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn const_(const_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let name_l = const_t.loc();
        let expression_l = name_l;

        Box::new(Node::Const(Const {
            scope: None,
            name: string_value(name_l, buffer),
            double_colon_l: None,
            name_l,
            expression_l,
        }))
    }

    pub(crate) fn const_global(colon2_t: Token<'a>, name_t: Token<'a>) -> Box<Node<'a>> {
        todo!()
    }

    // Assignments
    pub(crate) fn assignable(node: Box<Node<'a>>) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn const_op_assignable(node: Box<Node<'a>>) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn assign(lhs: Box<Node<'a>>, op_t: Token<'a>, rhs: Box<Node<'a>>) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn op_assign(
        lhs: Box<Node<'a>>,
        op_t: Token<'a>,
        rhs: Box<Node<'a>>,
    ) -> Box<Node<'a>> {
        todo!()
    }

    // Class and module definition

    // Method (un)definition

    // Formatl arguments

    // Method calls

    // Logical operations: and, or

    // Conditionals

    // Case matching

    // Loops

    // Keywords

    // BEGIN, END
    pub(crate) fn preexe(
        preexe_t: Token<'a>,
        lbrace_t: Token<'a>,
        body: Option<Box<Node<'a>>>,
        rbrace_t: Token<'a>,
    ) -> Box<Node<'a>> {
        let keyword_l = preexe_t.loc();
        let begin_l = lbrace_t.loc();
        let end_l = rbrace_t.loc();
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
        postexe_t: Token<'a>,
        lbrace_t: Token<'a>,
        body: Option<Box<Node<'a>>>,
        rbrace_t: Token<'a>,
    ) -> Box<Node<'a>> {
        let keyword_l = postexe_t.loc();
        let begin_l = lbrace_t.loc();
        let end_l = rbrace_t.loc();
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
        rescue_t: Token<'a>,
        exc_list: Option<Vec<Node<'a>>>,
        exc_var: Option<(Token<'a>, Box<Node<'a>>)>,
        then_t: Option<Token<'a>>,
        body: Option<Box<Node<'a>>>,
    ) -> Box<Node<'a>> {
        let exc_list = exc_list.map(|exc_list| Self::array(None, exc_list, None));
        todo!()
    }

    // Expression grouping
    pub(crate) fn compstmt(statements: Vec<Node<'a>>) -> Box<Node<'a>> {
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

    pub(crate) fn begin(
        begin_t: Token<'a>,
        statements: Vec<Node<'a>>,
        end_t: Token<'a>,
    ) -> Box<Node<'a>> {
        let begin_l = begin_t.loc();
        let end_l = end_t.loc();
        Box::new(Node::Begin(Begin {
            statements,
            begin_l: Some(begin_l),
            end_l: Some(end_l),
            expression_l: begin_l.join(&end_l),
        }))
    }

    pub(crate) fn group(nodes: Vec<Node<'a>>) -> Box<Node<'a>> {
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

    pub(crate) fn group_with_trailing_comma(
        nodes: Vec<Node<'a>>,
        trailing_comma: Token<'a>,
    ) -> Box<Node<'a>> {
        todo!()
    }

    // Pattern matching

    pub(crate) fn def_method() -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn def_endless_method() -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn def_singleton() -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn def_endless_singleton() -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn undef(undef_t: Token<'a>, names: Vec<Node<'a>>) -> Box<Node<'a>> {
        debug_assert!(!names.is_empty());

        let keyword_l = undef_t.loc();
        let expression_l = keyword_l.join(names.last().unwrap().expression());
        Box::new(Node::Undef(Undef {
            names,
            keyword_l,
            expression_l,
        }))
    }

    pub(crate) fn alias(
        alias_t: Token<'a>,
        to: Box<Node<'a>>,
        from: Box<Node<'a>>,
    ) -> Box<Node<'a>> {
        let keyword_l = alias_t.loc();
        let expression_l = keyword_l.join(from.expression());
        Box::new(Node::Alias(Alias {
            to,
            from,
            keyword_l,
            expression_l,
        }))
    }
}

// Loc helpers

fn nodes_locs(nodes: &[Node]) -> (Loc, Loc, Loc) {
    debug_assert!(nodes.len() > 0);

    let begin = nodes.first().unwrap().expression().start;
    let end = nodes.last().unwrap().expression().end;

    let begin_l = Loc {
        start: begin,
        end: begin + 1,
    };
    let end_l = Loc {
        start: end,
        end: end + 1,
    };
    let expression_l = begin_l.join(&end_l);

    (begin_l, end_l, expression_l)
}
