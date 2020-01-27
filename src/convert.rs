use typenum::Unsigned;
use crate::{Fractional, Float, ConstFraction};

pub trait Convert<T> {
    type Output;
}

pub struct UInt;
pub struct Fraction;

pub trait ToPrimitive<R> {
    const VALUE: R;
}

macro_rules! impl_number {
    ($tr:ident @ $($ty:ty:$id:ident),*) => {
        $(
            impl<T: $tr> ToPrimitive<$ty> for T {
                const VALUE: $ty = T::$id;
            }
        )*
    };
}

impl_number! {
    Unsigned @ u8:U8, u16:U16, u32:U32, u64:U64, usize:USIZE,
    i8:I8, i16:I16, i32:I32, i64:I64, isize:ISIZE
}

impl_number! {
    Float @ f32:F32, f64:F64
}

impl<R: Fractional> ToPrimitive<ConstFraction> for R {
    const VALUE: ConstFraction = ConstFraction::new(<R::Num as Unsigned>::U64, <R::Denom as Unsigned>::U64);
}
