use crate::scalar::Scalar;

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
