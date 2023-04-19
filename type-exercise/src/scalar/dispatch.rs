use crate::macros::for_all_types;
use crate::TypeMismatch;

macro_rules! define_scalar_impl {
    ($({ $Name:ident, $Variant:ident, $Array:ident, $ArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum ScalarImpl {
            $(
                $Variant($Owned),
            )*
        }
    };
}

for_all_types! { define_scalar_impl }

macro_rules! define_scalar_ref_impl {
    ($({ $Name:ident, $Variant:ident, $Array:ident, $ArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum ScalarRefImpl<'a> {
            $(
                $Variant($Ref),
            )*
        }
    };
}

for_all_types! { define_scalar_ref_impl }

macro_rules! impl_scalar_conversion {
    ($({ $Name:ident, $Variant:ident, $Array:ident, $ArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        $(
            impl From<$Owned> for ScalarImpl {
                fn from(value: $Owned) -> Self {
                    Self::$Variant(value)
                }
            }

            impl TryFrom<ScalarImpl> for $Owned {
                type Error = TypeMismatch;
                fn try_from(value: ScalarImpl) -> Result<Self, Self::Error> {
                    match value {
                        ScalarImpl::$Variant(this) => Ok(this),
                        _ => Err(TypeMismatch),
                    }
                }
            }

            impl<'a> From<$Ref> for ScalarRefImpl<'a> {
                fn from(value: $Ref) -> Self {
                    Self::$Variant(value)
                }
            }

            impl<'a> TryFrom<ScalarRefImpl<'a>> for $Ref {
                type Error = TypeMismatch;
                fn try_from(value: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
                    match value {
                        ScalarRefImpl::$Variant(this) => Ok(this),
                        _ => Err(TypeMismatch),
                    }
                }
            }
        )*
    };
}

for_all_types! { impl_scalar_conversion }
