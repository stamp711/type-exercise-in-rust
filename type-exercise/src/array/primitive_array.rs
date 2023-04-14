use bitvec::vec::BitVec;

use super::{Array, ArrayBuilder, Scalar, ScalarRef};

impl PrimitiveType for i32 {}
impl PrimitiveType for f64 {}
pub type I32Array = PrimitiveArray<i32>;
pub type F64Array = PrimitiveArray<f64>;

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
