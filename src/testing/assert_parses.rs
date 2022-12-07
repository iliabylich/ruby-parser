macro_rules! assert_parses_rule {
    ($rule:ty, $src:expr, $expected:expr) => {{
        use crate::{
            parser::base::{ParseResult, Rule},
            Node, Parser,
        };

        let mut parser = Parser::new($src).debug();
        type TestRule = $rule;
        assert!(TestRule::starts_now(&mut parser));
        let parsed: ParseResult<Box<Node>> = TestRule::parse(&mut parser);

        let ast;
        match parsed {
            Ok(node) => ast = node,
            Err(err) => {
                eprintln!("{:?}", err);
                panic!("expected Ok(node), got Err()")
            }
        }

        let expected: &str = $expected;
        dbg!(&ast);
        assert_eq!(ast.inspect(0), expected.trim_start().trim_end());
        assert!(parser.lexer.buffer.is_eof());
        assert_eq!(parser.lexer.string_literals.size(), 0);
    }};
}
pub(crate) use assert_parses_rule;
