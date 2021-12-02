use std::fmt::Debug;
use std::mem;

use crate::types::Value;

pub trait SuperType<T: Value> {
    fn upcast_internal(v: &T) -> Self;
}

#[doc(hidden)]
/// Trait asserting that `Self` is a transparent wrapper around `Self::Inner`
/// with identical representation and may be safely transmuted.
///
/// # Safety
/// `Self` must be `#[repr(transparent)]` with a field `Self::Inner`. The
/// `TransparentWrapper::into_inner` must be implemented to avoid dependently
/// sized types.
pub unsafe trait TransparentWrapper: Sized {
    type Inner: Debug + Copy;

    fn into_inner(self) -> Self::Inner;

    fn wrap_ref(s: &Self::Inner) -> &Self {
        unsafe { mem::transmute(s) }
    }

    fn wrap_mut(s: &mut Self::Inner) -> &mut Self {
        unsafe { mem::transmute(s) }
    }
}
