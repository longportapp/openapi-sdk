use std::{ffi::c_void, fmt::Debug};

use crate::{
    async_call::{CAsyncResult, ToAsyncResult},
    types::ToFFI,
};

pub(crate) struct CCow<T: ToFFI> {
    #[allow(dead_code)]
    owned_value: Box<T>,
    ffi_value: T::FFIType,
}

impl<T: ToFFI> CCow<T> {
    pub(crate) fn new<Q>(value: Q) -> Self
    where
        T: From<Q>,
    {
        let owned_value: Box<T> = Box::new(From::from(value));
        let ffi_value = owned_value.to_ffi_type();
        Self {
            owned_value,
            ffi_value,
        }
    }
}

impl<T: ToFFI + Debug> Debug for CCow<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.owned_value.fmt(f)
    }
}

impl<T: ToFFI> ToFFI for CCow<T> {
    type FFIType = *const T::FFIType;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        &self.ffi_value
    }
}

impl<T: ToFFI> ToAsyncResult for CCow<T> {
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        (self.to_ffi_type(), 1).to_async_result(ctx)
    }
}
