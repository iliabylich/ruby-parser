use std::num::NonZeroUsize;

use crate::lexer::numbers::{scan, Buffer};

pub(crate) fn dot_number_suffix(buffer: &Buffer, start: usize) -> Option<NonZeroUsize> {
    if buffer.byte_at(start) != Some(b'.') {
        return None;
    }

    match scan::decimal(buffer, start + 1) {
        None => None,
        Some(len) => {
            // track leading '.'
            NonZeroUsize::new(len.get() + 1)
        }
    }
}

pub(crate) fn e_suffix(buffer: &Buffer, start: usize) -> Option<NonZeroUsize> {
    if !matches!(buffer.byte_at(start), Some(b'e' | b'E')) {
        return None;
    }

    // consume optional sign
    let mut sign_length = 0;
    if matches!(buffer.byte_at(start + 1), Some(b'-' | b'+')) {
        sign_length = 1;
    }

    match scan::decimal(buffer, start + 1 + sign_length) {
        None => None,
        Some(len) => {
            // track leading sign and 'e'
            NonZeroUsize::new(len.get() + 1 + sign_length)
        }
    }
}

pub(crate) fn r_suffix(buffer: &Buffer, start: usize) -> Option<NonZeroUsize> {
    if buffer.byte_at(start) != Some(b'r') {
        return None;
    }
    // TODO: check lookahead (like 'rescue')
    NonZeroUsize::new(1)
}

pub(crate) fn i_suffix(buffer: &Buffer, start: usize) -> Option<NonZeroUsize> {
    if buffer.byte_at(start) != Some(b'i') {
        return None;
    }
    // TODO: check lookahead (like 'if')
    NonZeroUsize::new(1)
}
