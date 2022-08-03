macro_rules! assert_parses {
    ($f:ident, $src:expr, $expected:expr) => {{
        use crate::{
            parser::{ParseResult, Parser},
            Node,
        };

        let src: &[u8] = $src;
        let mut parser = Parser::new(src).debug();
        let parsed: ParseResult<Box<Node>> = parser.$f();

        let ast;
        match parsed {
            Ok(node) => ast = node,
            Err(err) => {
                eprintln!("{}", err.render());
                panic!("expected Ok(node), got Err()")
            }
        }

        let expected: &str = $expected;
        assert_eq!(ast.inspect(0), expected.trim_start().trim_end());
    }};
}
pub(crate) use assert_parses;
