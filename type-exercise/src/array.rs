pub mod primitive_array;
pub mod string_array;

pub use primitive_array::*;
pub use string_array::*;

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
pub trait Array: Send + Sync + Sized + 'static // + TryFrom<ArrayImpl> + Into<ArrayImpl>
{
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

    fn from_slice(slice: &[Option<Self::RefItem<'_>>]) -> Self {
        let mut builder = Self::Builder::with_capacity(slice.len());
        for item in slice {
            builder.push(*item);
        }
        builder.finish()
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

/// All variants of [`Array`].
pub enum ArrayImpl {
    Int32(I32Array),
    Float64(F64Array),
    String(StringArray),
}

impl TryFrom<ArrayImpl> for StringArray {
    type Error = ();
    fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
        match array {
            ArrayImpl::String(this) => Ok(this),
            _ => Err(()),
        }
    }
}

impl From<StringArray> for ArrayImpl {
    fn from(array: StringArray) -> Self {
        Self::String(array)
    }
}

impl TryFrom<ArrayImpl> for I32Array {
    type Error = ();
    fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
        match array {
            ArrayImpl::Int32(this) => Ok(this),
            _ => Err(()),
        }
    }
}

impl From<I32Array> for ArrayImpl {
    fn from(array: I32Array) -> Self {
        Self::Int32(array)
    }
}

impl TryFrom<ArrayImpl> for F64Array {
    type Error = ();
    fn try_from(array: ArrayImpl) -> Result<Self, Self::Error> {
        match array {
            ArrayImpl::Float64(this) => Ok(this),
            _ => Err(()),
        }
    }
}

impl From<F64Array> for ArrayImpl {
    fn from(array: F64Array) -> Self {
        Self::Float64(array)
    }
}

/// All variants of [`Scalar`].
pub enum ScalarImpl {
    Int32(i32),
    Float64(f64),
    String(String),
}

/// All variants of [`ScalarRef`].
pub enum ScalarRefImpl<'a> {
    Int32(i32),
    Float64(f64),
    String(&'a str),
}

#[cfg(test)]
mod tests {
    use super::primitive_array::I32Array;
    use super::string_array::StringArray;
    use super::{Array, ArrayBuilder, ArrayImpl};

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

    fn add_i32(a: i32, b: i32) -> i32 {
        a + b
    }

    fn add_i32_vec(i1: &I32Array, i2: &I32Array) -> I32Array {
        let mut builder = <I32Array as Array>::Builder::with_capacity(i1.len());
        for (a, b) in i1.iter().zip(i2.iter()) {
            let sum = a.and_then(|a| b.map(|b| add_i32(a, b)));
            builder.push(sum);
        }
        builder.finish()
    }

    fn add_i32_wrapper(i1: ArrayImpl, i2: ArrayImpl) -> Result<ArrayImpl, ()> {
        Ok(add_i32_vec(&i1.try_into()?, &i2.try_into()?).into())
    }

    #[test]
    fn test_add_array() {
        check_array_eq::<I32Array>(
            &add_i32_wrapper(
                I32Array::from_slice(&[Some(1), Some(2), Some(3), None]).into(),
                I32Array::from_slice(&[Some(1), Some(2), None, Some(4)]).into(),
            )
            .unwrap()
            .try_into()
            .unwrap(),
            &[Some(2), Some(4), None, None],
        );

        let result = add_i32_wrapper(
            StringArray::from_slice(&[Some("1"), Some("2"), Some("3"), None]).into(),
            I32Array::from_slice(&[Some(1), Some(2), None, Some(4)]).into(),
        );
        assert!(result.is_err());
    }
}
