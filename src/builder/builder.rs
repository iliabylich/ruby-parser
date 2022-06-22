use crate::{
    builder::{CString, Constructor, RustConstructor},
    lexer::buffer::Buffer,
    nodes::*,
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

fn value(loc: Loc, buffer: &Buffer) -> CString {
    CString::from(buffer.slice(loc.start, loc.end).unwrap())
}

impl<C: Constructor> Builder<C> {
    pub(crate) fn gvar<'a>(token: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        let loc = token.loc();
        let name = value(loc, buffer);
        unsafe { node_ptr_to_box!(C::gvar_node(name, loc)) }
    }

    pub(crate) fn back_ref<'a>(token: Token<'a>, buffer: &Buffer) -> Box<Node<'a>> {
        let loc = token.loc();
        let name = value(loc, buffer);
        unsafe { node_ptr_to_box!(C::back_ref_node(name, loc)) }
    }

    pub(crate) fn alias<'a>(
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
