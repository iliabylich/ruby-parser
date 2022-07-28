macro_rules! scan_while_matches_pattern {
    ($buffer:expr, $start:expr, $pattern:pat) => {{
        use crate::buffer::LookaheadResult;

        let mut end = $start;
        loop {
            match $buffer.byte_at(end) {
                Some($pattern) => {
                    end += 1;
                }
                _ => {
                    break;
                }
            }
        }
        if ($start == end) {
            LookaheadResult::None
        } else {
            LookaheadResult::Some {
                length: end - $start,
            }
        }
    }};
}
pub(crate) use scan_while_matches_pattern;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum LookaheadResult {
    None,
    Some { length: usize },
}

#[test]
fn test_scan_while_matches_pattern() {
    use crate::buffer::Buffer;

    let buffer = Buffer::new(b"abcdefghijk");
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'a'..=b'd'),
        LookaheadResult::Some { length: 4 }
    );
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'a'..=b'z'),
        LookaheadResult::Some { length: 11 }
    );
    assert_eq!(
        scan_while_matches_pattern!(buffer, 0, b'0'..=b'9'),
        LookaheadResult::None
    );
}
