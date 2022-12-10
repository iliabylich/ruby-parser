use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, Maybe1, Maybe2, ParseResult, Rule, SeparatedBy},
        OptElse, OptRescue, Preexe, TermT, Value,
    },
    Node, Parser, Token, TokenKind,
};

pub(crate) struct TopStmts;
impl Rule for TopStmts {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Stmts::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let stmts = Stmts::parse(parser).unwrap();
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::group(stmts)))
        }
    }
}

pub(crate) struct Compstmt;
impl Rule for Compstmt {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Stmts::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let stmts = Stmts::parse(parser).unwrap();
        let _opt_terms = OptTerms::parse(parser).unwrap();
        if stmts.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Builder::group(stmts)))
        }
    }
}

pub(crate) struct Bodystmt;
impl Rule for Bodystmt {
    type Output = Option<Box<Node>>;

    fn starts_now(parser: &mut Parser) -> bool {
        Compstmt::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let compound_stmt = Compstmt::parse(parser).unwrap();
        let rescue_bodies = OptRescue::parse(parser).unwrap();
        let opt_else = OptElse::parse(parser).unwrap();
        type OptEnsure = Maybe2<ExactToken<{ TokenKind::kENSURE as u8 }>, Compstmt>;
        let opt_ensure = OptEnsure::parse(parser).unwrap();
        Ok(Some(Builder::begin_body(
            compound_stmt,
            rescue_bodies,
            opt_else,
            opt_ensure,
        )))
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

        fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
            Bodystmt::parse(parser).map(|maybe_node| maybe_node.unwrap())
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        Terms::parse(parser).unwrap();
        Ok(())
    }
}

struct Stmts;
impl Rule for Stmts {
    type Output = Vec<Node>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        let mut stmts = vec![];
        loop {
            match ValueOrPreexe::parse(parser).unwrap() {
                ValueOrPreexe::Value(stmt) => stmts.push(*stmt),
                ValueOrPreexe::Term => continue,
                ValueOrPreexe::None => break,
            }
        }
        Ok(stmts)
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

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        if Value::starts_now(parser) {
            Ok(Self::Value(Value::parse(parser).unwrap()))
        } else if Preexe::starts_now(parser) {
            Ok(Self::Value(Preexe::parse(parser).unwrap()))
        } else if Terms::starts_now(parser) {
            parser.skip_token();
            Ok(Self::Term)
        } else {
            Ok(Self::None)
        }
    }
}

struct Terms;
impl Rule for Terms {
    type Output = ();

    fn starts_now(parser: &mut Parser) -> bool {
        TermT::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> ParseResult<Self::Output> {
        type SemiT = ExactToken<{ TokenKind::tSEMI as u8 }>;
        let _ = SeparatedBy::<TermT, SemiT>::parse(parser).unwrap();
        Ok(())
    }
}
