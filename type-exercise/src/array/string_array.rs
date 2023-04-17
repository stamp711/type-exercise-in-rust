use bitvec::vec::BitVec;

use crate::array::{Array, ArrayBuilder};

pub struct StringArray {
    /// The flattened data of string.
    data: Vec<u8>,
    /// Offsets of each string in the data flat array, plus an extra offset at the end.
    offsets: Vec<usize>,
    /// The null bitmap of this array.
    bitmap: BitVec,
}

impl StringArray {
    fn get(&self, idx: usize) -> Option<&str> {
        if self.bitmap[idx] {
            let start = self.offsets[idx];
            let end = self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[start..end]) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.offsets.len() - 1
    }
}

impl Array for StringArray {
    type OwnedItem = String;
    type RefItem<'a> = &'a str;
    type Builder = StringArrayBuilder;
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

pub type StringArrayBuilder = StringArray;

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut this = Self {
            data: Vec::new(),
            offsets: Vec::with_capacity(capacity + 1),
            bitmap: BitVec::with_capacity(capacity),
        };
        this.offsets.push(0);
        this
    }

    fn push(&mut self, item: Option<<Self::Array as Array>::RefItem<'_>>) {
        self.bitmap.push(item.is_some());
        if let Some(item) = item {
            self.data.extend_from_slice(item.as_bytes());
        }
        self.offsets.push(self.data.len());
    }

    fn finish(self) -> Self::Array {
        self
    }
}
