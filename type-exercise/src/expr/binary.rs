use super::Expression;
use crate::array::{Array, ArrayBuilder, ArrayImpl};
use crate::scalar::Scalar;
use crate::TypeMismatch;

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

    pub fn eval(&self, i1: &ArrayImpl, i2: &ArrayImpl) -> anyhow::Result<ArrayImpl>
    where
        for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
        for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
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

impl<I1: Array, I2: Array, O: Array, F> Expression for BinaryExpression<I1, I2, O, F>
where
    for<'a> &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    for<'a> &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    F: Fn(I1::RefItem<'_>, I2::RefItem<'_>) -> O::OwnedItem,
    O: Into<ArrayImpl>,
{
    fn eval_expr(&self, data: &[&ArrayImpl]) -> anyhow::Result<ArrayImpl> {
        if data.len() != 2 {
            anyhow::bail!("BinaryExpression requires 2 arguments");
        }
        self.eval(data[0], data[1])
    }
}
