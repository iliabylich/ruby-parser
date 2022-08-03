#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Loc {
    pub start: usize,
    pub end: usize,
}

impl Loc {
    /// Converts location to a range
    pub fn to_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }

    /// Returns size of the `Loc` (i.e. `end - start`)
    pub fn size(&self) -> usize {
        self.end - self.start
    }

    /// Returns a new `Loc` with given `start` and current `end`
    pub fn with_start(&self, start: usize) -> Loc {
        Self {
            start: start,
            end: self.end,
        }
    }

    /// Returns a new `Loc` with given `end` and current `start`
    pub fn with_end(&self, end: usize) -> Loc {
        Self {
            start: self.start,
            end,
        }
    }

    /// Adds given `delta` to `start`
    pub fn adjust_start(&self, delta: i32) -> Loc {
        let start: i32 = self
            .start
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let start: usize = (start + delta)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self {
            start: start,
            end: self.end,
        }
    }

    /// Adds given `delta` to `end`
    pub fn adjust_end(&self, d: i32) -> Loc {
        let end: i32 = self
            .end
            .try_into()
            .expect("failed to convert location to i32 (is it too big?)");
        let end: usize = (end + d)
            .try_into()
            .expect("failed to convert location to usize (is it negative?)");
        Self {
            start: self.start,
            end,
        }
    }

    /// Returns a new `Loc` with the same `start`, but adjusted `end`,
    /// so that its size is equal to given `new_size`
    pub fn resize(&self, new_size: usize) -> Loc {
        self.with_end(self.start + new_size)
    }

    /// Joins two `Loc`s by choosing `min(start)` + `max(end)`
    pub fn join(&self, other: &Self) -> Loc {
        Self {
            start: std::cmp::min(self.start, other.start),
            end: std::cmp::max(self.end, other.end),
        }
    }

    pub(crate) fn maybe_join(&self, other: &Option<Loc>) -> Loc {
        match other.as_ref() {
            Some(other) => self.join(other),
            None => *self,
        }
    }

    /// Returns true if `Loc` is empty (i.e. `start` == `end`)
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    // pub(crate) fn start_line_col(&self, _input: &Buffer) -> Option<(usize, usize)> {
    //     input.line_col_for_pos(self.start)
    // }

    // pub(crate) fn expand_to_line(&self, _input: &Buffer) -> Option<(usize, Loc)> {
    //     let (start_line, _) = self.start_line_col(input)?;
    //     let line_no = start_line;
    //     let line = input.line_at(line_no);
    //     Some((line_no, Self { start: line.start, line.line_end())))
    // }

    // /// Returns source code of the current `Loc` on a given `Input`
    // pub fn source(&self, _input: &Buffer) -> Option<String> {
    //     let bytes = input.substr_at(self.start, self.end)?;
    //     Some(String::from_utf8_lossy(bytes).into_owned())
    // }

    // pub(crate) fn print(&self, name: &str) {
    //     println!(
    //         "{}{} {}",
    //         " ".repeat(self.start),
    //         "~".repeat(self.size()),
    //         name
    //     )
    // }
}

impl std::fmt::Debug for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}...{}", self.start, self.end))
    }
}

macro_rules! loc {
    ($start:expr, $end:expr) => {
        crate::Loc {
            start: $start,
            end: $end,
        }
    };
}
pub(crate) use loc;

#[test]
fn test_to_range() {
    assert_eq!(Loc { start: 10, end: 20 }.to_range(), 10..20)
}

#[test]
fn test_fmt() {
    assert_eq!(format!("{:?}", Loc { start: 10, end: 20 }), "10...20")
}

#[test]
fn test_is_empty() {
    assert!(Loc { start: 1, end: 1 }.is_empty());
    assert!(!Loc { start: 1, end: 2 }.is_empty());
}
