use crate::{PrimitiveArray, PrimitiveType, StringArray};

pub trait Array: Send + Sync + Sized + 'static {
    type RefItem: ?Sized;
    /// Retrieve a reference to value.
    fn get_ref(&self, idx: usize) -> Option<&Self::RefItem>;
    fn len(&self) -> usize;
    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator {
            array: self,
            pos: 0,
        }
    }
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem = T;
    fn get_ref(&self, idx: usize) -> Option<&Self::RefItem> {
        if self.bitmap[idx] {
            Some(&self.data[idx])
        } else {
            None
        }
    }
    fn len(&self) -> usize {
        self.len()
    }
}

impl Array for StringArray {
    type RefItem = str;
    fn get_ref(&self, idx: usize) -> Option<&Self::RefItem> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

/// An iterator that iterators on any [`Array`] type.
pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<&'a A::RefItem>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.array.len() {
            let item = self.array.get_ref(self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
