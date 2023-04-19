pub use binary::*;
pub use cmp::*;
pub use string::*;

use crate::array::{ArrayImpl, BoolArray, I32Array, StringArray};

mod binary;
mod cmp;
mod string;

pub trait Expression {
    /// Evaluate the expression with the given input arrays.
    fn eval_expr(&self, data: &[&ArrayImpl]) -> anyhow::Result<ArrayImpl>;
}

/// All supported expression functions.
pub enum ExpressionFunc {
    CmpLt,
    CmpLe,
    CmpGt,
    CmpGe,
    CmpEq,
    CmpNe,
    StrContains,
    StrConcat,
}

pub fn build_binary_expression(f: ExpressionFunc) -> Box<dyn Expression> {
    use ExpressionFunc::*;

    match f {
        CmpLt => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_lt::<i32, i32, i32>,
        )),
        CmpLe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_le::<i32, i32, i32>,
        )),
        CmpGt => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_gt::<i32, i32, i32>,
        )),
        CmpGe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_ge::<i32, i32, i32>,
        )),
        CmpEq => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_eq::<i32, i32, i32>,
        )),
        CmpNe => Box::new(BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_ne::<i32, i32, i32>,
        )),
        StrContains => {
            Box::new(BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains))
        }
        StrConcat => {
            Box::new(BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array::*;
    use crate::expr::*;
    use crate::scalar::ScalarRefImpl;

    #[test]
    fn test_build_str_contains() {
        let expr = build_binary_expression(ExpressionFunc::StrContains);

        for _ in 0..10 {
            let result = expr
                .eval_expr(&[
                    &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
                    &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
                ])
                .unwrap();
            assert_eq!(result.get(0).unwrap(), ScalarRefImpl::Bool(true));
            assert_eq!(result.get(1).unwrap(), ScalarRefImpl::Bool(false));
            assert!(result.get(2).is_none());
        }
    }
}
