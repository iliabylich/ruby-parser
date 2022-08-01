use crate::{
    builder::{Builder, Constructor},
    nodes::*,
    token::Token,
    Node,
};

impl<C: Constructor> Builder<C> {
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
