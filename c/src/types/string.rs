use std::os::raw::c_char;

use crate::types::ToFFI;

#[derive(Debug)]
pub(crate) struct CString(std::ffi::CString);

impl From<String> for CString {
    fn from(s: String) -> Self {
        unsafe { CString(std::ffi::CString::from_vec_unchecked(s.into_bytes())) }
    }
}

impl ToFFI for CString {
    type FFIType = *const c_char;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        self.0.as_ptr()
    }
}
