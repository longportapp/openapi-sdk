use std::{ffi::c_void, fmt::Debug};

use crate::{
    async_call::{CAsyncResult, ToAsyncResult},
    types::ToFFI,
};

pub(crate) struct CVec<T: ToFFI> {
    #[allow(dead_code)]
    owned_values: Vec<T>,
    ffi_values: Vec<T::FFIType>,
}

impl<T: ToFFI + Debug> Debug for CVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.owned_values.fmt(f)
    }
}

impl<Q, T: ToFFI + From<Q>> From<Vec<Q>> for CVec<T> {
    fn from(values: Vec<Q>) -> Self {
        let owned_values = values.into_iter().map(From::from).collect::<Vec<_>>();
        let ffi_values = owned_values.iter().map(ToFFI::to_ffi_type).collect();

        Self {
            owned_values,
            ffi_values,
        }
    }
}

impl<T: ToFFI> CVec<T> {
    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.ffi_values.len()
    }
}

impl<T: ToFFI> ToFFI for CVec<T> {
    type FFIType = *const T::FFIType;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        self.ffi_values.as_ptr()
    }
}

impl<T: ToFFI> ToAsyncResult for CVec<T> {
    fn to_async_result(&self, ctx: *const c_void) -> CAsyncResult {
        (self.ffi_values.as_ptr(), self.len()).to_async_result(ctx)
    }
}
