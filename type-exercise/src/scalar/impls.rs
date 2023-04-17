use crate::array::*;
use crate::macros::for_all_primitive_types;
use crate::scalar::*;

pub trait PrimitiveType: Copy + Send + Sync + std::fmt::Debug + 'static {}

impl<T: PrimitiveType> Scalar for T {
    type ArrayType = PrimitiveArray<T>;
    type RefType<'a> = T;

    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        *self
    }
}

impl<T: PrimitiveType> ScalarRef<'_> for T {
    type ArrayType = PrimitiveArray<T>;
    type ScalarType = T;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        *self
    }
}

macro_rules! impl_scalar_for_primitive_types {
    ($({ $Name:ident, $Variant:ident, $Array:ident, $ArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        $(
            impl PrimitiveType for $Owned {}
        )*
    };
}

for_all_primitive_types! { impl_scalar_for_primitive_types }

impl Scalar for String {
    type ArrayType = StringArray;
    type RefType<'a> = &'a str;
    fn as_scalar_ref(&self) -> Self::RefType<'_> {
        self.as_str()
    }
}

impl<'a> ScalarRef<'a> for &'a str {
    type ArrayType = StringArray;
    type ScalarType = String;
    fn to_owned_scalar(&self) -> Self::ScalarType {
        self.to_string()
    }
}
