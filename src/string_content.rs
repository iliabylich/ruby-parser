#[derive(Debug, Clone, Eq)]
pub struct StringContent {
    bytes: Vec<u8>,
}

impl From<&[u8]> for StringContent {
    fn from(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }
}

impl From<&str> for StringContent {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<String> for StringContent {
    fn from(s: String) -> Self {
        Self::from(s.into_bytes())
    }
}

impl From<Vec<u8>> for StringContent {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl From<TokenValue> for StringContent {
    fn from(token_value: TokenValue) -> Self {
        Self::from(token_value.into_bytes())
    }
}

impl PartialEq for StringContent {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl<const N: usize> PartialEq<[u8; N]> for StringContent {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.as_bytes() == other
    }
}

use crate::token::TokenValue;

impl StringContent {
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_bytes()).into_owned()
    }

    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.as_bytes()).unwrap()
    }
}
