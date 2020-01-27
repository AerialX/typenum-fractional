pub type ConstFraction = Fraction<u64>;

#[derive(Copy, Clone, Debug)]
pub struct Fraction<T> {
    pub num: T,
    pub denom: T,
}

impl<T> Fraction<T> {
    pub const fn new(num: T, denom: T) -> Self {
        Self {
            num,
            denom,
        }
    }
}

#[cfg(feature = "unstable")]
impl Fraction<u64> {
    pub const fn const_invert(self) -> Self {
        Self {
            num: self.denom,
            denom: self.num,
        }
    }

    pub const fn const_from_scalar(num: u64) -> Self {
        Self {
            num,
            denom: 1,
        }
    }

    pub const fn const_reduce(self) -> Self {
        let gcd = gcd(self.num, self.denom);
        Self {
            num: self.num / gcd,
            denom: self.denom / gcd,
        }
    }

    pub const fn const_mul(self, rhs: Self) -> Self {
        Self {
            num: self.num * rhs.num,
            denom: self.denom * rhs.denom,
        }.const_reduce()
    }

    pub const fn const_div(self, rhs: Self) -> Self {
        self.const_mul(rhs.const_invert())
    }

    pub const fn const_value(self) -> f64 {
        self.num as f64 / self.denom as f64
    }

    pub const fn const_truncate(self) -> u64 {
        self.num / self.denom
    }
}

#[cfg(feature = "unstable")]
const fn gcd(lhs: u64, rhs: u64) -> u64 {
    if lhs == 0 {
        rhs
    } else if rhs == 0 {
        lhs
    } else {
        gcd(rhs, lhs % rhs)
    }
}


/* const generics graveyard
pub struct CB<const T: bool> {
    _internal: PhantomData<bool>,
}

impl Bit for CB<{true}> {
    const U8: u8 = 1;
    const BOOL: bool = true;

    fn to_u8() -> u8 { Self::U8 }
    fn to_bool() -> bool { Self::BOOL }
}

impl Bit for CB<{false}> {
    const U8: u8 = 0;
    const BOOL: bool = false;

    fn to_u8() -> u8 { Self::U8 }
    fn to_bool() -> bool { Self::BOOL }
}

impl<If, Else> Conditional<If, Else> for CB<{false}> {
    type Output = Else;
}

impl<If, Else> Conditional<If, Else> for CB<{true}> {
    type Output = If;
}

pub struct CUInt<const T: u64> {
    _internal: PhantomData<u64>,
}

impl<const T: u64> Unsigned for CUInt<T> {
    const U8: u8 = T as u8;
    const U16: u16 = T as u16;
    const U32: u32 = T as u32;
    const U64: u64 = T as u64;
    const USIZE: usize = T as usize;
    const I8: i8 = T as i8;
    const I16: i16 = T as i16;
    const I32: i32 = T as i32;
    const I64: i64 = T as i64;
    const ISIZE: isize = T as isize;

    fn to_u8() -> u8 { Self::U8 }
    fn to_u16() -> u16 { Self::U16 }
    fn to_u32() -> u32 { Self::U32 }
    fn to_u64() -> u64 { Self::U64 }
    fn to_usize() -> usize { Self::USIZE }
    fn to_i8() -> i8 { Self::I8 }
    fn to_i16() -> i16 { Self::I16 }
    fn to_i32() -> i32 { Self::I32 }
    fn to_i64() -> i64 { Self::I64 }
    fn to_isize() -> isize { Self::ISIZE }
}

impl<const T: u64> NonZero for CUInt<T> where Self: typenum::IsGreater<U0> { }

impl<O, OGL, RHS: Unsigned, const T: u64> Cmp<RHS> for CUInt<T> where
    CB<{T > RHS::U64}>: Conditional<typenum::Greater, typenum::Less, Output=OGL>,
    CB<{T == RHS::U64}>: Conditional<typenum::Equal, OGL, Output=O>
{
    type Output = O;
}

impl Convert<B0> for CB<{false}> {
    type Output = B0;
}

impl Convert<B0> for CB<{true}> {
    type Output = B1;
}
*/
