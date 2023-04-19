use crate::array::*;
use crate::macros::for_all_types;
use crate::scalar::*;
use crate::TypeMismatch;

macro_rules! array_dispatch {
    ($({ $Name:ident, $Variant:ident, $Array:ty, $ArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        /// All variants of [`Array`].
        pub enum ArrayImpl {
            $(
                $Variant($Array),
            )*
        }

        /// All variants of [`ArrayBuilder`].
        pub enum ArrayBuilderImpl {
            $(
                $Variant($ArrayBuilder),
            )*
        }

        // Dispatch methods for ArrayImpl.
        impl ArrayImpl {
            /// Get the value at `idx`.
            pub fn get(&self, idx: usize) -> Option<ScalarRefImpl<'_>> {
                match self {
                    $(
                        Self::$Variant(this) => this.get(idx).map(ScalarRefImpl::$Variant),
                    )*
                }
            }

            /// Get the length of the array.
            pub fn len(&self) -> usize {
                match self {
                    $(
                        Self::$Variant(this) => this.len(),
                    )*
                }
            }

            /// Check if the array is empty.
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }
        }

        // Dispatch methods for ArrayBuilderImpl.
        impl ArrayBuilderImpl {
            /// Append a value to the array.
            pub fn push(&mut self, value: Option<ScalarRefImpl<'_>>) {
                match (self, value) {
                    $(
                        (Self::$Variant(this), Some(ScalarRefImpl::$Variant(v))) => this.push(Some(v)),
                        (Self::$Variant(this), None) => this.push(None),
                    )*
                    _ => { panic!("Unexpected type") }
                }
            }

            /// Finish building the array.
            pub fn finish(self) -> ArrayImpl {
                match self {
                    $(
                        Self::$Variant(this) => ArrayImpl::$Variant(this.finish()),
                    )*
                }
            }
        }

        // Conversion between ArrayImpl and Array.
        $(
            impl std::convert::TryFrom<ArrayImpl> for $Array {
                type Error = TypeMismatch;
                fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
                    match array {
                        ArrayImpl::$Variant(this) => Ok(this),
                        _ => Err(TypeMismatch),
                    }
                }
            }

            impl<'a> std::convert::TryFrom<&'a ArrayImpl> for &'a $Array {
                type Error = TypeMismatch;
                fn try_from(array: &'a ArrayImpl) -> Result<Self, Self::Error> {
                    match array {
                        ArrayImpl::$Variant(this) => Ok(this),
                        _ => Err(TypeMismatch),
                    }
                }
            }

            impl From<$Array> for ArrayImpl {
                fn from(array: $Array) -> Self {
                    Self::$Variant(array)
                }
            }
        )*

        // Conversion between ArrayBuilderImpl and ArrayBuilder.
        $(
            impl std::convert::TryFrom<ArrayBuilderImpl> for $ArrayBuilder {
                type Error = TypeMismatch;
                fn try_from(array: ArrayBuilderImpl) -> Result<Self, Self::Error> {
                    match array {
                        ArrayBuilderImpl::$Variant(this) => Ok(this),
                        _ => Err(TypeMismatch),
                    }
                }
            }

            impl From<$ArrayBuilder> for ArrayBuilderImpl {
                fn from(array: $Array) -> Self {
                    Self::$Variant(array)
                }
            }
        )*
    };
}

for_all_types! { array_dispatch }
