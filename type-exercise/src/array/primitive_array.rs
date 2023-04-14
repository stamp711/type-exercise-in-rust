use bitvec::vec::BitVec;

use super::{Array, ArrayBuilder};

pub trait PrimitiveType: Copy + Send + Sync + 'static {}

pub struct PrimitiveArray<T: PrimitiveType> {
    /// The actual data of this array.
    data: Vec<T>,
    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: PrimitiveType> PrimitiveArray<T> {
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

impl<T: PrimitiveType> ArrayBuilder for PrimitiveArrayBuilder<T> {
    type Array = PrimitiveArray<T>;

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
