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

    /// The builder type of this array.
    type Builder: ArrayBuilder<Array = Self>;

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

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;
    fn get<'s>(&'s self, idx: usize) -> Option<Self::RefItem<'s>> {
        self.get(idx)
    }
    fn len(&self) -> usize {
        self.len()
    }
}

type StringArrayBuilder = StringArray;

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
        self.offsets.push(item.map_or(0, |i| i.len()));
        if let Some(item) = item {
            self.data.extend_from_slice(item.as_bytes());
        }
    }

    fn finish(self) -> Self::Array {
        self
    }
}

impl Array for StringArray {
    type RefItem<'a> = &'a str;
    type Builder = StringArrayBuilder;
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

pub trait ArrayBuilder {
    type Array: Array;

    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, item: Option<<Self::Array as Array>::RefItem<'_>>);
    fn finish(self) -> Self::Array;
}

#[allow(unused)]
fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
    assert_eq!(i1.len(), i2.len(), "size mismatch");
    let mut builder = O::Builder::with_capacity(i1.len());
    for (i1, i2) in i1.iter().zip(i2.iter()) {
        //   builder.push(sql_func(i1, i2));
    }
    builder.finish()
}
