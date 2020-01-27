use core::ops::Rem;
use typenum::{UInt, UTerm, Unsigned, Bit, NonZero, Mod};

pub type Gcd<LHS, RHS> = <LHS as GreatestCommon<RHS>>::Output;

pub trait GreatestCommon<RHS> {
    type Output;
}

impl<LHS> GreatestCommon<LHS> for UTerm {
    type Output = LHS;
}

impl<U: Unsigned, B: Bit> GreatestCommon<UTerm> for UInt<U, B> {
    type Output = Self;
}

/// A hack around trait issues
#[doc(hidden)]
pub trait NonZeroPrivate { }
impl<U: Unsigned, B: Bit> NonZeroPrivate for UInt<U, B> { }

impl<U: Unsigned, B: Bit, RHS> GreatestCommon<RHS> for UInt<U, B> where
    RHS: GreatestCommon<Mod<Self, RHS>> + NonZeroPrivate,
    Self: Rem<RHS> + NonZero,
{
    type Output = Gcd<RHS, Mod<Self, RHS>>;
}
