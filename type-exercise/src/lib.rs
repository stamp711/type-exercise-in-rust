#![feature(generic_associated_types)]
#![allow(clippy::result_unit_err)]

use array::{Array, ArrayImpl};

pub mod array;
pub(crate) mod macros;
pub mod scalar;

#[allow(unused_variables)]
pub fn eval_binary<'a, I1: Array, I2: Array>(
    i1: &'a ArrayImpl,
    i2: &'a ArrayImpl,
) -> Result<ArrayImpl, ()>
where
    &'a I1: TryFrom<&'a ArrayImpl, Error = ()>,
    &'a I2: TryFrom<&'a ArrayImpl, Error = ()>,
{
    let i1: &I1 = i1.try_into()?;
    let i2: &I2 = i2.try_into()?;
    todo!()
}
