use crate::transactions::{steps::Steps, ParseError};

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
                    "{offset}TOKEN ({weight}) expected {expected:?}, got {got:?} (at {at})",
                    offset = ws_offset(level),
                    weight = self.weight(),
                    expected = expected,
                    got = got,
                    at = loc.start
                )
            }
            Self::OneOfError { name, variants } => {
                format!(
                    "{offset}ONEOF ({weight}) {rule}\n{variants}",
                    offset = ws_offset(level),
                    weight = self.weight(),
                    rule = name,
                    variants = variants
                        .iter()
                        .map(|v| v.render_with_level(level + 1))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            }
            Self::SeqError {
                name,
                steps: Steps(steps),
                error,
            } => {
                format!(
                    "{offset}SEQUENCE ({weight}) {name} (got {steps:?})\n{error}",
                    offset = ws_offset(level),
                    weight = self.weight(),
                    name = name,
                    steps = steps,
                    error = error.render_with_level(level + 1)
                )
            }
        }
    }
}

fn ws_offset(level: usize) -> String {
    " ".repeat(level * 4)
}
