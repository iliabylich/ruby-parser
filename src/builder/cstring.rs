use crate::string_content::StringContent;

#[repr(C)]
pub struct CString {
    ptr: *mut u8,
    length: usize,
}

impl From<String> for CString {
    fn from(s: String) -> Self {
        CString::from(s.into_bytes())
    }
}

impl From<Vec<u8>> for CString {
    fn from(mut bytes: Vec<u8>) -> Self {
        bytes.shrink_to_fit();
        let mut bytes = bytes.into_boxed_slice();
        let (ptr, length) = (bytes.as_mut_ptr(), bytes.len());
        std::mem::forget(bytes);
        Self { ptr, length }
    }
}

impl From<&[u8]> for CString {
    fn from(bytes: &[u8]) -> Self {
        CString::from(bytes.to_vec())
    }
}

impl From<CString> for StringContent<'_> {
    fn from(cstring: CString) -> Self {
        StringContent::from(Vec::from(cstring))
    }
}

impl From<CString> for Vec<u8> {
    fn from(cstring: CString) -> Self {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(cstring.ptr, cstring.length);
            slice.to_vec()
        }
    }
}

impl From<CString> for String {
    fn from(cstring: CString) -> Self {
        unsafe {
            let bytes = Vec::from(cstring);
            String::from_utf8_unchecked(bytes)
        }
    }
}
