use bitvec::vec::BitVec;

use crate::array::{Array, ArrayBuilder};
use crate::macros::for_all_primitive_types;
use crate::scalar::{PrimitiveType, Scalar, ScalarRef};

pub struct PrimitiveArray<T> {
    /// The actual data of this array.
    data: Vec<T>,
    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: for<'a> Scalar<ArrayType = Self, RefType<'a> = T>,
    T: for<'a> ScalarRef<'a, ArrayType = Self, ScalarType = T>,
{
    type OwnedItem = T;
    type RefItem<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> PrimitiveArray<T>
where
    T: PrimitiveType,
{
    fn get(&self, idx: usize) -> Option<T> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

type PrimitiveArrayBuilder<T> = PrimitiveArray<T>;

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: for<'a> Scalar<ArrayType = Self, RefType<'a> = T>,
    T: for<'a> ScalarRef<'a, ArrayType = Self, ScalarType = T>,
{
    type Array = Self;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, item: Option<<Self::Array as Array>::RefItem<'_>>) {
        self.bitmap.push(item.is_some());
        if let Some(value) = item {
            self.data.push(value);
        }
    }

    fn finish(self) -> Self::Array {
        self
    }
}

macro_rules! define_primitive_array {
    ($({ $Name:ident, $Variant:ident, $Array:ident, $ArrayBuilder:ident, $Owned:ty, $Ref:ty }),*) => {
        $(
            pub type $Array = PrimitiveArray<$Owned>;
            pub type $ArrayBuilder = PrimitiveArrayBuilder<$Owned>;
        )*
    };
}

for_all_primitive_types! {define_primitive_array}
