use array::{Array, ArrayImpl};

pub mod array;
pub mod expr;
pub(crate) mod macros;
pub mod scalar;

#[derive(Debug, thiserror::Error)]
#[error("type mismatch")]
pub struct TypeMismatch;

#[allow(unused_variables)]
pub fn eval_binary<'a, I1: Array, I2: Array>(
    i1: &'a ArrayImpl,
    i2: &'a ArrayImpl,
) -> Result<ArrayImpl, TypeMismatch>
where
    &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
{
    let i1: &I1 = i1.try_into()?;
    let i2: &I2 = i2.try_into()?;
    todo!()
}

#[cfg(test)]
#[allow(unused)]
pub(crate) mod test_util {
    use crate::array::{Array, ArrayBuilder};

    /// Build an array from a vector of data
    pub(crate) fn build_array_from_vec<A: Array>(items: &[Option<A::RefItem<'_>>]) -> A {
        let mut builder = A::Builder::with_capacity(items.len());
        for item in items {
            builder.push(*item);
        }
        builder.finish()
    }

    /// Test if an array has the same content as a vector
    pub(crate) fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
    where
        A::RefItem<'a>: PartialEq,
    {
        for (a, b) in array.iter().zip(vec.iter()) {
            assert_eq!(&a, b);
        }
    }
}
