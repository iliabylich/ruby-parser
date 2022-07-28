#[cfg(test)]
macro_rules! assert_parses_with_error {
    ($f:ident, $src:expr, $expected:literal) => {{
        use crate::parser::RustParser;

        let src: &[u8] = $src;
        let mut parser = RustParser::new(src).debug();
        let result = parser.$f();

        match result {
            Ok(value) => panic!("expected Err(...), got Ok({:?})", value),
            Err(err) => {
                println!("== Actual error ==\n{}", err.render());
                assert_eq!(err.render(), $expected.trim_start().trim_end())
            }
        }

        parser
    }};
}
#[cfg(test)]
pub(crate) use assert_parses_with_error;
