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
        todo!()
    }
    pub(crate) fn true_(true_t: Token) -> Box<Node> {
        todo!()
    }
    pub(crate) fn false_(false_t: Token) -> Box<Node> {
        todo!()
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
    fn __line__(line_t: Token) -> Box<Node> {
        todo!()
    }
    fn __file__(file_t: Token) -> Box<Node> {
        todo!()
    }
    fn __encoding__(encoding_t: Token) -> Box<Node> {
        todo!()
    }

    // Strings

    // Symbols

    // Executable string

    // Regular expressions

    // Arrays

    // Hashes

    // Ranges

    // Access
    // fn self_(token: Token) -> Box<Node>;
    pub(crate) fn lvar(token: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = token.loc();
        Box::new(Node::Lvar(Lvar {
            name: string_value(loc, buffer),
            expression_l: loc,
        }))
    }
    pub(crate) fn ivar(token: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        todo!()
    }
    pub(crate) fn gvar(token: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let loc = token.loc();
        let name = cstring_value(loc, buffer);
        unsafe { node_ptr_to_box!(C::gvar_node(name, loc)) }
    }
    pub(crate) fn cvar(token: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        todo!()
    }
    pub(crate) fn back_ref(token: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        let loc = token.loc();
        let name = cstring_value(loc, buffer);
        unsafe { node_ptr_to_box!(C::back_ref_node(name, loc)) }
    }
    pub(crate) fn nth_ref(token: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        todo!()
    }

    pub(crate) fn const_(name_t: Token<'a>, buffer: &Buffer<'a>) -> Box<Node<'a>> {
        let name_l = name_t.loc();
        let expression_l = name_l;

        Box::new(Node::Const(Const {
            scope: None,
            name: string_value(name_l, buffer),
            double_colon_l: None,
            name_l,
            expression_l,
        }))
    }

    // Assignments

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

    // Exception handling

    // Expression grouping

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
