/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static {
    /// The reference item of this array.
    type RefItem<'a>;

    /// The builder type of this array.
    type Builder: ArrayBuilder<Array = Self>;

    /// Retrieve a reference to value.
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

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

pub trait ArrayBuilder {
    type Array: Array;

    fn with_capacity(capacity: usize) -> Self;
    fn push(&mut self, item: Option<<Self::Array as Array>::RefItem<'_>>);
    fn finish(self) -> Self::Array;
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

mod primitive_array;
mod string_array;

#[cfg(test)]
mod test {
    use super::{Array, ArrayBuilder};

    #[allow(unused)]
    fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
        assert_eq!(i1.len(), i2.len(), "size mismatch");
        let mut builder = O::Builder::with_capacity(i1.len());
        for (i1, i2) in i1.iter().zip(i2.iter()) {
            //   builder.push(sql_func(i1, i2));
        }
        builder.finish()
    }
}
