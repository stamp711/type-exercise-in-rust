mod dispatch;
mod impls;

pub use dispatch::*;
pub use impls::*;

use crate::array::Array;

/// An owned single value.
///  
/// For example, `i32`, `String` both implements [`Scalar`].
pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static {
    type ArrayType: for<'a> Array<OwnedItem = Self, RefItem<'a> = Self::RefType<'a>>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a {
    type ArrayType: Array<RefItem<'a> = Self, OwnedItem = Self::ScalarType>;
    type ScalarType: Scalar<RefType<'a> = Self, ArrayType = Self::ArrayType>;
    fn to_owned_scalar(&self) -> Self::ScalarType;
}
