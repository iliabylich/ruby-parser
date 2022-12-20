use crate::{
    builder::Builder,
    parser::{
        base::{at_most_one_is_true, ExactToken, Maybe1, Maybe3, Rule},
        Bodystmt, FnameT, Params, TermT, Value, VarRef,
    },
    token::{Token, TokenKind},
    Node, Parser,
};

pub(crate) struct MethodDef;
impl Rule for MethodDef {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        DefHead::starts_now(parser)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let def_head = DefHead::parse(parser);
        let args = MethodDefArgs::parse(parser);
        let body = Bodystmt::parse(parser);
        let end_t = parser.expect_token(TokenKind::kEND).unwrap();
        match def_head {
            DefHead::DefnHead { def_t, name_t } => {
                Builder::def_method(def_t, name_t, args, body, end_t, parser.buffer())
            }
            DefHead::DefsHead {
                def_t,
                definee,
                dot_t,
                name_t,
            } => Builder::def_singleton(
                def_t,
                definee,
                dot_t,
                name_t,
                args,
                body,
                end_t,
                parser.buffer(),
            ),
        }
    }
}
#[test]
fn test_instance_method_def() {
    crate::testing::assert_parses_rule!(
        MethodDef,
        b"def foo; 42; end",
        r#"
s(:def, "foo", nil,
  s(:int, "42"))
            "#
    )
}
#[test]
fn test_singleton_method_def() {
    crate::testing::assert_parses_rule!(
        MethodDef,
        b"def self.foo; 42; end",
        r#"
s(:defs,
  s(:self), "foo", nil,
  s(:int, "42"))
            "#
    )
}

pub(crate) struct EndlessMethodDef<T>
where
    T: Rule<Output = Box<Node>>,
{
    _t: std::marker::PhantomData<T>,
}
impl<T> Rule for EndlessMethodDef<T>
where
    T: Rule<Output = Box<Node>>,
{
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kDEF)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

enum DefHead {
    DefnHead {
        def_t: Token,
        name_t: Token,
    },

    DefsHead {
        def_t: Token,
        definee: Box<Node>,
        dot_t: Token,
        name_t: Token,
    },
}
impl Rule for DefHead {
    type Output = Self;

    fn starts_now(parser: &mut Parser) -> bool {
        parser.current_token().is(TokenKind::kDEF)
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        let def_t = parser.take_token();

        if FnameT::starts_now(parser) || Singleton::starts_now(parser) {
            // unclear, depends on the presence of '.' or '::'
            // because 'def self; end' is a valid construction

            let as_token = parser.current_token();
            let as_node = Singleton::parse(parser);

            if DotOrColonT::starts_now(parser) {
                // singleton method
                let definee = as_node;
                let dot_t = DotOrColonT::parse(parser);
                let name_t = FnameT::parse(parser);
                Self::DefsHead {
                    def_t,
                    definee,
                    dot_t,
                    name_t,
                }
            } else {
                // instance method
                let name_t = as_token;
                Self::DefnHead { def_t, name_t }
            }
        } else if FnameT::starts_now(parser) {
            // obvious instance method
            // like `def +(other)`
            let name_t = FnameT::parse(parser);
            Self::DefnHead { def_t, name_t }
        } else if Singleton::starts_now(parser) {
            // obvious singleton method
            // like `def self.foo`
            let definee = Singleton::parse(parser);
            let dot_t = DotOrColonT::parse(parser);
            let name_t = FnameT::parse(parser);
            Self::DefsHead {
                def_t,
                definee,
                dot_t,
                name_t,
            }
        } else {
            unreachable!()
        }
    }
}

struct MethodDefArgs;
impl Rule for MethodDefArgs {
    type Output = Option<Box<Node>>;

    fn starts_now(_parser: &mut Parser) -> bool {
        true
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        type MaybeParams = Maybe1<Params>;

        let begin_t;
        let args;
        let end_t;

        if parser.current_token().is(TokenKind::tLPAREN) {
            begin_t = Some(parser.take_token());
            args = MaybeParams::parse(parser).unwrap_or_default();
            end_t = Some(parser.expect_token(TokenKind::tRPAREN).unwrap());
        } else {
            begin_t = None;
            args = MaybeParams::parse(parser).unwrap_or_default();
            end_t = None;
            if !args.is_empty() {
                TermT::parse(parser);
            }
        }

        Builder::args(begin_t, args, end_t)
    }
}

struct EndlessMethodArgs;
impl Rule for EndlessMethodArgs {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        todo!()
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        todo!()
    }
}

struct Singleton;
impl Rule for Singleton {
    type Output = Box<Node>;

    fn starts_now(parser: &mut Parser) -> bool {
        at_most_one_is_true([
            VarRef::starts_now(parser),
            parser.current_token().is(TokenKind::tLPAREN),
        ])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if VarRef::starts_now(parser) {
            VarRef::parse(parser)
        } else if parser.current_token().is(TokenKind::tLPAREN) {
            let lparen_t = parser.take_token();
            let value = Value::parse(parser);
            let rparen_t = parser.take_token();
            Builder::begin(lparen_t, vec![*value], rparen_t)
        } else {
            unreachable!()
        }
    }
}

struct DotOrColonT;
impl Rule for DotOrColonT {
    type Output = Token;

    fn starts_now(parser: &mut Parser) -> bool {
        let token = parser.current_token();
        at_most_one_is_true([token.is(TokenKind::tDOT), token.is(TokenKind::tCOLON2)])
    }

    fn parse(parser: &mut Parser) -> Self::Output {
        if Self::starts_now(parser) {
            parser.take_token()
        } else {
            unreachable!()
        }
    }
}
