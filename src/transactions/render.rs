use crate::transactions::ParseError;

#[allow(dead_code)]
impl ParseError {
    pub(crate) fn render(&self) -> String {
        self.render_with_level(0)
    }

    fn render_with_level(&self, level: usize) -> String {
        match self {
            Self::TokenError {
                lookahead: _lookahead,
                expected,
                got,
                loc,
            } => {
                format!(
                    "{offset}TOKEN expected {expected:?}, got {got:?} (at {at})",
                    offset = ws_offset(level),
                    expected = expected,
                    got = got,
                    at = loc.start
                )
            }
            Self::OneOfError { name, variants } => {
                format!(
                    "{offset}ONEOF {rule}\n{variants}",
                    offset = ws_offset(level),
                    rule = name,
                    variants = variants
                        .iter()
                        .map(|v| v.render_with_level(level + 1))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Self::SeqError { name, steps, error } => {
                format!(
                    "{offset}SEQUENCE {name} (got {steps:?})\n{error}",
                    offset = ws_offset(level),
                    name = name,
                    steps = steps,
                    error = error.render_with_level(level + 1)
                )
            }
            Self::None => {
                format!("{offset}NONE", offset = ws_offset(level))
            }
        }
    }
}

fn ws_offset(level: usize) -> String {
    " ".repeat(level * 4)
}

#[cfg(test)]
macro_rules! assert_err_eq {
    ($actual:expr, $expected:literal) => {
        match $actual {
            Ok(value) => panic!("expected Err(...), got Ok({:?}", value),
            Err(err) => assert_eq!(err.render(), $expected.trim_start().trim_end()),
        }
    };
}
#[cfg(test)]
pub(crate) use assert_err_eq;
