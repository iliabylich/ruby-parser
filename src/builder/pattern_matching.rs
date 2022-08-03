use crate::{
    builder::Builder,
    nodes::{MatchPattern, MatchPatternP},
    token::Token,
    Node,
};

impl Builder {
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

    pub(crate) fn in_pattern() -> Box<Node> {
        todo!("builder.in_pattern")
    }
    pub(crate) fn if_guard() -> Box<Node> {
        todo!("builder.if_guard")
    }
    pub(crate) fn unless_guard() -> Box<Node> {
        todo!("builder.unless_guard")
    }
    pub(crate) fn match_var() -> Box<Node> {
        todo!("builder.match_var")
    }
    pub(crate) fn match_hash_var() -> Box<Node> {
        todo!("builder.match_hash_var")
    }
    pub(crate) fn match_hash_var_from_str() -> Box<Node> {
        todo!("builder.match_hash_var_from_str")
    }
    pub(crate) fn match_rest() -> Box<Node> {
        todo!("builder.match_rest")
    }
    pub(crate) fn hash_pattern() -> Box<Node> {
        todo!("builder.hash_pattern")
    }
    pub(crate) fn array_pattern() -> Box<Node> {
        todo!("builder.array_pattern")
    }
    pub(crate) fn find_pattern() -> Box<Node> {
        todo!("builder.find_pattern")
    }
    pub(crate) fn const_pattern() -> Box<Node> {
        todo!("builder.const_pattern")
    }
    pub(crate) fn pin() -> Box<Node> {
        todo!("builder.pin")
    }
    pub(crate) fn match_alt() -> Box<Node> {
        todo!("builder.match_alt")
    }
    pub(crate) fn match_as() -> Box<Node> {
        todo!("builder.match_as")
    }
    pub(crate) fn match_nil_pattern() -> Box<Node> {
        todo!("builder.match_nil_pattern")
    }
    pub(crate) fn match_pair() -> Box<Node> {
        todo!("builder.match_pair")
    }
    pub(crate) fn match_label() -> Box<Node> {
        todo!("builder.match_label")
    }
}
