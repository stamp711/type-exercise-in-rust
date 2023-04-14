/// An owned single value.
///  
/// For example, `i32`, `String` both implements [`Scalar`].
pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static {
    type ArrayType: for<'a> Array<OwnedItem = Self, RefItem<'a> = Self::RefType<'a>>;
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayType>;
    fn as_scalar_ref(&self) -> Self::RefType<'_>;
}

pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a {
    type ArrayType: Array<RefItem<'a> = Self, OwnedItem = Self::ScalarType>;
    type ScalarType: Scalar<RefType<'a> = Self, ArrayType = Self::ArrayType>;
    fn to_owned_scalar(&self) -> Self::ScalarType;
}

/// [`Array`] is a collection of data of the same type.
pub trait Array: Send + Sync + Sized + 'static {
    /// The owned item of this array.
    type OwnedItem: for<'a> Scalar<ArrayType = Self, RefType<'a> = Self::RefItem<'a>>;

    /// The reference item of this array.
    type RefItem<'a>: ScalarRef<'a, ArrayType = Self, ScalarType = Self::OwnedItem>;

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

pub mod primitive_array;
pub mod string_array;

#[cfg(test)]
mod tests {
    use super::primitive_array::I32Array;
    use super::string_array::StringArray;
    use super::{Array, ArrayBuilder};

    // These are two examples of using generics over array.
    //
    // These functions work for all kinds of array, no matter fixed-length arrays like `I32Array`,
    // or variable-length ones like `StringArray`.

    /// Build an array from a vector of data
    fn build_array_from_vec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
        let mut builder = A::Builder::with_capacity(items.len());
        for item in items {
            builder.push(*item);
        }
        builder.finish()
    }

    /// Test if an array has the same content as a vector
    fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
    where
        A::RefItem<'a>: PartialEq,
    {
        for (a, b) in array.iter().zip(vec.iter()) {
            assert_eq!(&a, b);
        }
    }

    #[test]
    fn test_build_int32_array() {
        let data = vec![Some(1), Some(2), Some(3), None, Some(5)];
        let array = build_array_from_vec::<I32Array>(&data[..]);
        check_array_eq(&array, &data[..]);
    }

    #[test]
    fn test_build_string_array() {
        let data = vec![Some("1"), Some("2"), Some("3"), None, Some("5"), Some("")];
        let array = build_array_from_vec::<StringArray>(&data[..]);
        check_array_eq(&array, &data[..]);
    }
}
