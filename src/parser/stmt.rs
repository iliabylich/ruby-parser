use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, Maybe2, Rule, SeparatedBy},
        OptElse, OptRescue, Preexe, TermT, Value,
    },
    Node, Parser, TokenKind,
};

pub(crate) struct TopStmts;
impl Rule for TopStmts {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Stmts::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let stmts = Stmts::parse(parser);
        if stmts.is_empty() {
            None
        } else {
            Some(Builder::group(stmts))
        }
    }
}

pub(crate) struct Compstmt;
impl Rule for Compstmt {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Stmts::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let stmts = Stmts::parse(parser);
        let _opt_terms = OptTerms::parse(parser);
        if stmts.is_empty() {
            None
        } else {
            Some(Builder::group(stmts))
        }
    }
}

pub(crate) struct Bodystmt;
impl Rule for Bodystmt {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Compstmt::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let compound_stmt = Compstmt::parse(parser);
        let rescue_bodies = OptRescue::parse(parser);
        let opt_else = OptElse::parse(parser);
        type OptEnsure = Maybe2<ExactToken<{ TokenKind::kENSURE as u8 }>, Compstmt>;
        let opt_ensure = OptEnsure::parse(parser);
        Some(Builder::begin_body(
            compound_stmt,
            rescue_bodies,
            opt_else,
            opt_ensure,
        ))
    }
}
#[test]
fn test_bodystmt() {
    struct GuaranteedBodyStmt;
    impl Rule for GuaranteedBodyStmt {
        type Output = Box<Node>;

        fn starts_now(_parser: &mut Parser) -> bool {
            true // irrelevant
        }

        fn parse(parser: &mut Parser) -> Self::Output {
            Bodystmt::parse(parser).unwrap()
        }
    }
    crate::testing::assert_parses_rule!(
        GuaranteedBodyStmt,
        b"42; rescue; 'resc'; else; 'els'; ensure; 'ens';",
        r#"
s(:ensure,
  s(:rescue,
    s(:int, "42"),
    s(:resbody, nil, nil,
      s(:str, "resc")),
    s(:str, "els")),
  s(:str, "ens"))
        "#
    )
}

pub(crate) struct OptTerms;
impl Rule for OptTerms {
    type Output = ();

    fn starts_now(parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        Terms::parse(parser);
    }
}

struct Stmts;
impl Rule for Stmts {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let mut stmts = vec![];
        loop {
            match ValueOrPreexe::parse(parser) {
                ValueOrPreexe::Value(stmt) => stmts.push(*stmt),
                ValueOrPreexe::Term => continue,
                ValueOrPreexe::None => break,
            }
        }
        stmts
    }
}

enum ValueOrPreexe {
    Value(Box<Node>),
    Term,
    None,
}
impl Rule for ValueOrPreexe {
    type Output = Self;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            Value::starts_now(parser),
            Preexe::starts_now(parser),
            Terms::starts_now(parser),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Value::starts_now(parser) {
            Self::Value(Value::parse(parser))
        } else if Preexe::starts_now(parser) {
            Self::Value(Preexe::parse(parser))
        } else if Terms::starts_now(parser) {
            parser.skip_token();
            Self::Term
        } else {
            Self::None
        }
    }
}

struct Terms;
impl Rule for Terms {
    type Output = ();

    fn starts_now(parser: &mut Parser) -> bool {
        TermT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        type SemiT = ExactToken<{ TokenKind::tSEMI as u8 }>;
        let _ = SeparatedBy::<TermT, SemiT>::parse(parser);
    }
}
