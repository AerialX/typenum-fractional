#![no_std]
#![cfg_attr(feature = "unstable", feature(const_fn, const_if_match))]

use core::marker::PhantomData;
use typenum::{
    Unsigned, NonZero, Bit,
    UTerm, UInt,
    Cmp,
    Prod, Quot, Compare,
    consts::*
};
use core::ops::{Mul, Div};
use const_default::ConstDefault;

mod float;
pub use float::Float;

pub mod convert;
pub use convert::{Convert, ToPrimitive};

pub mod conditional;
pub use conditional::{Conditional, If};

pub mod common;
pub use common::{GreatestCommon, Gcd};

pub mod consts;
pub use consts::ConstFraction;

pub type Truncated<T> = <T as Truncate>::Output;
pub type Reduced<T> = <T as Reduce>::Output;
pub type Invert<T> = <T as Reciprocal>::Output;

pub trait Fractional {
    type Num: Unsigned;
    type Denom: Unsigned + NonZero;
}

pub trait Reduce {
    type Output;
}

pub trait Truncate {
    type Output: Unsigned;
}

pub trait Reciprocal {
    type Output;
}

impl Fractional for UTerm {
    type Num = Self;
    type Denom = U1;
}

impl<U: Unsigned, B: Bit> Fractional for UInt<U, B> {
    type Num = Self;
    type Denom = U1;
}

pub struct Fraction<N, D> {
    _internal: PhantomData<(N, D)>,
}

impl<N: Unsigned, D: Unsigned + NonZero> Float for Fraction<N, D> {
    const F32: f32 = Self::F64 as f32;
    const F64: f64 = N::U64 as f64 / D::U64 as f64;
}

impl<N, D> Fraction<N, D> {
    pub const fn new() -> Self {
        Self {
            _internal: PhantomData,
        }
    }
}

impl<N: Unsigned, D: Unsigned + NonZero> Fractional for Fraction<N, D> {
    type Num = N;
    type Denom = D;
}

impl<N: Unsigned + NonZero, D: Unsigned + NonZero> NonZero for Fraction<N, D> { }

impl<N, D> Copy for Fraction<N, D> { }

impl<N, D> Clone for Fraction<N, D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<N, D> ConstDefault for Fraction<N, D> {
    const DEFAULT: Self = Fraction::new();
}

impl<N, D> Default for Fraction<N, D> {
    fn default() -> Self {
        Fraction::new()
    }
}

impl<O, R: Fractional> Truncate for R where
    R::Num: Div<R::Denom, Output=O>,
    O: Unsigned,
{
    type Output = O;
}

impl<O, OR, N: Unsigned, D: Unsigned + NonZero, RHS: Fractional> Cmp<RHS> for Fraction<N, D> where
    N: Mul<RHS::Denom, Output=O>,
    RHS::Num: Mul<D, Output=OR>,
    O: Cmp<OR>,
{
    type Output = Compare<O, OR>;

    fn compare<P: typenum::private::InternalMarker>(&self, _: &RHS) -> Self::Output {
        unsafe { ::core::mem::MaybeUninit::uninit().assume_init() }
    }
}

impl<N: Unsigned + GreatestCommon<D>, D: Unsigned + NonZero> Reduce for Fraction<N, D> where
    N: Div<Gcd<N, D>>,
    D: Div<Gcd<N, D>>,
{
    type Output = Fraction<Quot<N, Gcd<N, D>>, Quot<D, Gcd<N, D>>>;
}

impl<O, N: Unsigned + Mul<R::Num>, D: Unsigned + NonZero + Mul<R::Denom>, R: Fractional> Mul<R> for Fraction<N, D> where
    Fraction<Prod<N, R::Num>, Prod<D, R::Denom>>: Reduce<Output=O>,
    O: ConstDefault,
{
    type Output = O;

    fn mul(self, _: R) -> Self::Output {
        ConstDefault::DEFAULT
    }
}

impl<N: Unsigned> Unsigned for Fraction<N, U1> {
    const U8: u8 = N::U8;
    const U16: u16 = N::U16;
    const U32: u32 = N::U32;
    const U64: u64 = N::U64;
    const USIZE: usize = N::USIZE;
    const I8: i8 = N::I8;
    const I16: i16 = N::I16;
    const I32: i32 = N::I32;
    const I64: i64 = N::I64;
    const ISIZE: isize = N::ISIZE;

    fn to_u8() -> u8 { N::to_u8() }
    fn to_u16() -> u16 { N::to_u16() }
    fn to_u32() -> u32 { N::to_u32() }
    fn to_u64() -> u64 { N::to_u64() }
    fn to_usize() -> usize { N::to_usize() }
    fn to_i8() -> i8 { N::to_i8() }
    fn to_i16() -> i16 { N::to_i16() }
    fn to_i32() -> i32 { N::to_i32() }
    fn to_i64() -> i64 { N::to_i64() }
    fn to_isize() -> isize { N::to_isize() }
}

impl<N: Unsigned> Convert<convert::UInt> for Fraction<N, U1> {
    type Output = N;
}

impl<U: Unsigned, B: Bit> Convert<convert::Fraction> for UInt<U, B> {
    type Output = Fraction<Self, U1>;
}

impl<R: Fractional> Reciprocal for R where
    R::Num: NonZero,
{
    type Output = Fraction<R::Denom, R::Num>;
}

impl<O, N: Unsigned, D: Unsigned + NonZero, R: Fractional> Div<R> for Fraction<N, D> where
    R: Reciprocal,
    Self: Mul<Invert<R>, Output=O>,
    O: ConstDefault,
{
    type Output = O;

    fn div(self, _: R) -> Self::Output {
        ConstDefault::DEFAULT
    }
}

// TODO: Add, Sub

#[cfg(test)]
mod tests {
    use super::{Fraction, Gcd, Truncated};
    use typenum::{Prod, Quot, assert_type_eq};
    use typenum::consts::*;

    #[test]
    fn gcd() {
        assert_type_eq!(Gcd<U0, U1>, U1);
        assert_type_eq!(Gcd<U1, U0>, U1);
        assert_type_eq!(Gcd<U1, U1>, U1);
        assert_type_eq!(Gcd<U1, U9>, U1);
        assert_type_eq!(Gcd<U2, U4>, U2);
        assert_type_eq!(Gcd<U4, U6>, U2);
    }

    #[test]
    fn fractional_ops() {
        assert_type_eq!(Prod<Fraction<U1, U2>, Fraction<U1, U4>>, Fraction<U1, U8>);
        assert_type_eq!(Prod<Fraction<U1, U2>, Fraction<U60, U1>>, Fraction<U30, U1>);
        assert_type_eq!(Quot<Fraction<U1, U2>, Fraction<U60, U1>>, Fraction<U1, U120>);
    }

    #[test]
    fn scalars() {
        assert_type_eq!(Prod<Fraction<U3, U4>, U11>, Fraction<U33, U4>);
        assert_type_eq!(Truncated<Prod<Fraction<U3, U4>, U11>>, U8);
    }
}
