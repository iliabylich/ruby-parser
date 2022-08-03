use crate::{
    buffer::Buffer,
    builder::CString,
    loc::{loc, Loc},
    string_content::StringContent,
    token::{Token, TokenKind},
    Node,
};

pub(crate) fn node_ptr_to_box(ptr: *mut std::ffi::c_void) -> Box<Node> {
    unsafe { Box::from_raw(ptr as *mut Node) }
}

pub(crate) fn cstring_value(loc: Loc, buffer: &Buffer) -> CString {
    CString::from(buffer.slice(loc.start, loc.end).unwrap())
}
pub(crate) fn string_value(loc: Loc, buffer: &Buffer) -> StringContent {
    StringContent::from(buffer.slice(loc.start, loc.end).unwrap())
}

pub(crate) fn maybe_loc(token: &Option<Token>) -> Option<Loc> {
    token.map(|t| t.loc)
}

pub(crate) fn nodes_locs(nodes: &[Node]) -> (Loc, Loc, Loc) {
    debug_assert!(nodes.len() > 0);

    let begin = nodes.first().unwrap().expression().start;
    let end = nodes.last().unwrap().expression().end;

    let begin_l = loc!(begin, begin + 1);
    let end_l = loc!(end, end + 1);
    let expression_l = begin_l.join(&end_l);

    (begin_l, end_l, expression_l)
}

pub(crate) fn collection_map(
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

pub(crate) fn collection_expr(nodes: &[Node]) -> Option<Loc> {
    let lhs = nodes.first().map(|node| *node.expression());
    let rhs = nodes.last().map(|node| *node.expression());
    join_maybe_locs(&lhs, &rhs)
}

pub(crate) fn join_maybe_locs(lhs: &Option<Loc>, rhs: &Option<Loc>) -> Option<Loc> {
    match (lhs, rhs) {
        (None, None) => None,
        (None, Some(rhs)) => Some(*rhs),
        (Some(lhs), None) => Some(*lhs),
        (Some(lhs), Some(rhs)) => Some(lhs.join(rhs)),
    }
}

pub(crate) fn maybe_node_expr(node: &Option<&Node>) -> Option<Loc> {
    node.map(|node| *node.expression())
}

pub(crate) fn maybe_boxed_node_expr(node: &Option<Box<Node>>) -> Option<Loc> {
    node.as_deref().map(|node| *node.expression())
}

pub(crate) fn is_heredoc(begin_t: &Option<Token>) -> bool {
    if let Some(begin_t) = begin_t.as_ref() {
        return begin_t.kind == TokenKind::tHEREDOC_BEG;
    }
    false
}

pub(crate) fn heredoc_map(
    begin_t: &Option<Token>,
    nodes: &[Node],
    end_t: &Option<Token>,
) -> (Loc, Loc, Loc) {
    let begin_t = begin_t.as_ref().expect("bug: begin_t must be Some");
    let end_t = end_t.as_ref().expect("heredoc must have end_t");

    let heredoc_body_l = collection_expr(nodes).unwrap_or_else(|| end_t.loc);
    let expression_l = begin_t.loc;
    let heredoc_end_l = end_t.loc;

    (heredoc_body_l, heredoc_end_l, expression_l)
}

// Regexp heleprs
pub(crate) fn static_regexp_captures(node: &Node) -> Option<Vec<String>> {
    None
}
