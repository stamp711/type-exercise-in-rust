pub use cmp::*;
pub use string::*;

use crate::array::{Array, ArrayBuilder, ArrayImpl};
use crate::scalar::Scalar;

mod cmp;
mod string;

pub struct BinaryExpression<I1, I2, O, F> {
    func: F,
    _phantom: std::marker::PhantomData<(I1, I2, O)>,
}

impl<I1: Array, I2: Array, O: Array, F> BinaryExpression<I1, I2, O, F> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn eval(&self, i1: &ArrayImpl, i2: &ArrayImpl) -> Result<ArrayImpl, ()>
    where
        for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = ()>,
        for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = ()>,
        F: Fn(I1::RefItem<'_>, I2::RefItem<'_>) -> O::OwnedItem,
        O: Into<ArrayImpl>,
    {
        let i1: &I1 = i1.try_into()?;
        let i2: &I2 = i2.try_into()?;
        assert_eq!(i1.len(), i2.len(), "array length mismatch");

        let mut builder = O::Builder::with_capacity(i1.len());
        for (a, b) in i1.iter().zip(i2.iter()) {
            match (a, b) {
                (Some(a), Some(b)) => builder.push(Some((self.func)(a, b).as_scalar_ref())),
                _ => builder.push(None),
            }
        }

        Ok(builder.finish().into())
    }
}

#[cfg(test)]
mod test {
    use crate::array::*;
    use crate::expr::*;
    use crate::test_util::*;

    #[test]
    fn test_str_contains() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains);
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
                &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
            )
            .unwrap();
        check_array_eq::<BoolArray>(
            (&result).try_into().unwrap(),
            &[Some(true), Some(false), None],
        );
    }

    #[test]
    fn test_concat_string() {
        let expr = BinaryExpression::<StringArray, StringArray, StringArray, _>::new(str_concat);
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("aa"), Some("bb"), None]).into(),
                &StringArray::from_slice(&[Some("aa"), None, Some("cc")]).into(),
            )
            .unwrap();
        check_array_eq::<StringArray>((&result).try_into().unwrap(), &[Some("aaaa"), None, None]);
    }

    #[test]
    fn test_str_cmp_le() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(
            cmp_le::<String, String, String>,
        );
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("aa"), Some("bb"), None]).into(),
                &StringArray::from_slice(&[Some("aa"), None, Some("cc")]).into(),
            )
            .unwrap();
        check_array_eq::<BoolArray>((&result).try_into().unwrap(), &[Some(true), None, None]);
    }

    #[test]
    fn test_str_cmp_eq() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(
            cmp_eq::<String, String, String>,
        );
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("aa"), Some("bb"), None]).into(),
                &StringArray::from_slice(&[Some("aa"), None, Some("cc")]).into(),
            )
            .unwrap();
        check_array_eq::<BoolArray>((&result).try_into().unwrap(), &[Some(true), None, None]);
    }
}
