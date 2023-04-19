use crate::scalar::Scalar;

pub fn cmp_lt<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a, 'b> C::RefType<'a>: PartialOrd<C::RefType<'b>>,
{
    let c1 = i1.into();
    let c2 = i2.into();
    c1 < c2
}

pub fn cmp_le<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a, 'b> C::RefType<'a>: PartialOrd<C::RefType<'b>>,
{
    let c1 = i1.into();
    let c2 = i2.into();
    c1 <= c2
}

pub fn cmp_gt<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a, 'b> C::RefType<'a>: PartialOrd<C::RefType<'b>>,
{
    let c1 = i1.into();
    let c2 = i2.into();
    c1 >= c2
}

pub fn cmp_ge<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a, 'b> C::RefType<'a>: PartialOrd<C::RefType<'b>>,
{
    let c1 = i1.into();
    let c2 = i2.into();
    c1 >= c2
}

pub fn cmp_eq<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> C::RefType<'a>: Eq,
{
    let c1 = I1::upcast_ref(i1).into();
    let c2 = I2::upcast_ref(i2).into();
    c1 == c2
}

pub fn cmp_ne<I1: Scalar, I2: Scalar, C: Scalar>(i1: I1::RefType<'_>, i2: I2::RefType<'_>) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> C::RefType<'a>: Eq,
{
    let c1 = I1::upcast_ref(i1).into();
    let c2 = I2::upcast_ref(i2).into();
    c1 != c2
}

#[cfg(test)]
mod test {
    use crate::array::*;
    use crate::expr::*;
    use crate::test_util::*;

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
