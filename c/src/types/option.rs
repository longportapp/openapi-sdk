use std::fmt::Debug;

use crate::types::{CCow, ToFFI};

#[derive(Debug)]
pub(crate) struct COption<T: ToFFI> {
    value: Option<CCow<T>>,
}

impl<Q, T: ToFFI + From<Q>> From<Option<Q>> for COption<T> {
    fn from(value: Option<Q>) -> Self {
        Self {
            value: value.map(CCow::new),
        }
    }
}

impl<T: ToFFI> ToFFI for COption<T> {
    type FFIType = *const T::FFIType;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        match self.value.as_ref() {
            Some(value) => value.to_ffi_type(),
            None => std::ptr::null(),
        }
    }
}
