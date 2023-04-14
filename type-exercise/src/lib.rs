use bitvec::vec::BitVec;

pub trait PrimitiveType: Copy + Send + Sync + 'static {}

pub struct PrimitiveArray<T: PrimitiveType> {
    /// The actual data of this array.
    data: Vec<T>,
    /// The null bitmap of this array.
    bitmap: BitVec,
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

/// Variant: Uses `?Sized` for `RefItem` and return `Option<&Self::RefItem>` for `get` method.
mod borrow_not_sized;
/// Variant: Impl this trait on reference of array.
mod impl_on_array_ref;

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static {
    /// The reference item of this array.
    type RefItem<'a>;

    /// Retrieve a reference to value.
    fn get<'s>(&'s self, idx: usize) -> Option<Self::RefItem<'s>>;

    /// Number of items of array.
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the array.
    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator {
            array: self,
            pos: 0,
        }
    }
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem<'a> = T;

    fn get<'s>(&'s self, idx: usize) -> Option<Self::RefItem<'s>> {
        self.get(idx)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

impl Array for StringArray {
    type RefItem<'a> = &'a str;

    fn get<'s>(&'s self, idx: usize) -> Option<Self::RefItem<'s>> {
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

/// Blanket implementation of [`Iterator`] for [`ArrayIterator`].
impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.array.len() {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }
}
