#[derive(Debug, Clone, Eq)]
pub enum StringContent<'a> {
    Borrowed(&'a [u8]),
    Owned(Vec<u8>),
}

impl<'a> From<&'a [u8]> for StringContent<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self::Borrowed(bytes)
    }
}

impl<'a> From<&'a str> for StringContent<'a> {
    fn from(s: &'a str) -> Self {
        Self::Borrowed(s.as_bytes())
    }
}

impl<const N: usize> From<[u8; N]> for StringContent<'_> {
    fn from(bytes: [u8; N]) -> Self {
        Self::Owned(Vec::from(bytes))
    }
}

impl<const N: usize> From<&[u8; N]> for StringContent<'_> {
    fn from(bytes: &[u8; N]) -> Self {
        Self::Owned(Vec::from(bytes.to_owned()))
    }
}

impl From<Vec<u8>> for StringContent<'_> {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Owned(bytes)
    }
}

impl From<char> for StringContent<'_> {
    fn from(c: char) -> Self {
        let mut buf = vec![0; c.len_utf8()];
        c.encode_utf8(&mut buf);
        Self::from(buf)
    }
}

impl From<u8> for StringContent<'_> {
    fn from(byte: u8) -> Self {
        Self::from(vec![byte])
    }
}

impl<'a> StringContent<'a> {
    fn into_bytes(self) -> Vec<u8> {
        match self {
            StringContent::Borrowed(borrowed) => borrowed.to_vec(),
            StringContent::Owned(owned) => owned,
        }
    }

    fn as_bytes(&self) -> &[u8] {
        match self {
            StringContent::Borrowed(borrowed) => borrowed,
            StringContent::Owned(owned) => owned.as_slice(),
        }
    }
}

impl PartialEq for StringContent<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

use std::ops::{Add, AddAssign};

impl<'a> Add for StringContent<'a> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self.into_bytes();
        let mut rhs = rhs.into_bytes();
        lhs.append(&mut rhs);
        Self::Owned(lhs)
    }
}

impl<'a> AddAssign for StringContent<'a> {
    fn add_assign(&mut self, rhs: Self) {
        let mut bytes = match self {
            StringContent::Borrowed(borrowed) => borrowed.to_vec(),
            StringContent::Owned(bytes) => std::mem::take(bytes),
        };

        bytes.extend_from_slice(rhs.as_bytes());
        *self = StringContent::Owned(bytes)
    }
}
