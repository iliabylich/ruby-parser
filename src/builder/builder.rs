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

    // Executable string

    // Regular expressions

    // Arrays

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
    pub(crate) fn ivar(ivar_t: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        todo!()
    }
    pub(crate) fn gvar(gvar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = gvar_t.loc();
        let name = cstring_value(loc, buffer);
        unsafe { node_ptr_to_box!(C::gvar_node(name, loc)) }
    }
    pub(crate) fn cvar(cvar_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        todo!()
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

    // Expression grouping
    pub(crate) fn compstmt(statements: Vec<Node<'a>>) -> Option<Box<Node<'a>>> {
        if statements.is_empty() {
            None
        } else if statements.len() == 1 {
            Some(Box::new(statements.into_iter().next().unwrap()))
        } else {
            todo!()
            // let (begin_l, end_l, expression_l) = Self::collection_map(&None, &statements, &None);
            // Some(Box::new(Node::Begin(Begin {
            //     statements,
            //     begin_l,
            //     end_l,
            //     expression_l,
            // })))
        }
    }

    // Pattern matching

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
