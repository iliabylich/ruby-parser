use crate::parser::Parser;

pub(crate) fn parse<T, F>(mut f: F, src: &[u8]) -> (Parser, T)
where
    F: FnMut(&mut Parser) -> T,
{
    let mut parser = Parser::new(src).debug();
    let output = f(&mut parser);
    (parser, output)
}

macro_rules! assert_parses {
    ($f:expr, $src:expr, $expected:expr) => {{
        use crate::{
            parser::{ParseResult, Parser},
            testing::parse,
            Node,
        };

        let (parser, parsed): (Parser, ParseResult<Box<Node>>) = parse($f, $src);

        let ast;
        match parsed {
            Ok(node) => ast = node,
            Err(err) => {
                eprintln!("{}", err.render());
                panic!("expected Ok(node), got Err()")
            }
        }

        let expected: &str = $expected;
        dbg!(&ast);
        assert_eq!(ast.inspect(0), expected.trim_start().trim_end());
        assert!(parser.state.inner.buffer.is_eof())
    }};
}
pub(crate) use assert_parses;

macro_rules! assert_parses_some {
    ($f:expr, $src:expr, $expected:expr) => {{
        use crate::{
            parser::{ParseResult, Parser},
            testing::parse,
            Node,
        };

        let (parser, parsed): (Parser, ParseResult<Option<Box<Node>>>) = parse($f, $src);

        let ast = match parsed {
            Ok(Some(node)) => node,
            Ok(None) => {
                panic!("expected some AST to ber returned, got None")
            }
            Err(err) => {
                eprintln!("{}", err.render());
                panic!("expected Ok(node), got Err()")
            }
        };

        let expected: &str = $expected;
        dbg!(&ast);
        assert_eq!(ast.inspect(0), expected.trim_start().trim_end());
        assert!(parser.state.inner.buffer.is_eof())
    }};
}
pub(crate) use assert_parses_some;
