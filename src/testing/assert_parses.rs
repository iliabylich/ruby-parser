macro_rules! assert_parses {
    ($parsed:expr, $expected:expr) => {{
        use crate::{parser::ParseResult, Node};

        let parsed: ParseResult<Box<Node>> = $parsed;

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
