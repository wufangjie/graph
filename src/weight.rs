use std::fmt;
/// this module impl a weight trait, and a NoWeight zero sized type
use std::ops::{Add, AddAssign, Sub, SubAssign};

pub trait Weight:
    Clone
    + Copy
    + Default
    + Add<Output = Self>
    + Sub<Output = Self>
    + PartialOrd
    + AddAssign
    + SubAssign
    + fmt::Debug
{
    fn zero() -> Self {
        Default::default()
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }

    fn is_positive(&self) -> bool {
	*self > Self::zero()
    }

    fn is_negative(&self) -> bool {
	*self < Self::zero()
    }

    fn min(self, other: Self) -> Self {
	if self > other { other } else { self }
    }
}
/// since we can not implement Add, Sub trait for ()
/// I implement a zero sized type (i.e. NoWeight) by myself
#[derive(Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct NoWeight;

impl NoWeight {
    pub fn new() -> Self {
        Self
    }
}

impl Add for NoWeight {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self::Output {
        self
    }
}

impl Sub for NoWeight {
    type Output = Self;

    fn sub(self, _rhs: Self) -> Self::Output {
        self
    }
}

/// did not implement Neg implemented (u8, u16,...)
// impl Neg for NoWeight {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
// 	self
//     }
// }

impl AddAssign for NoWeight {
    fn add_assign(&mut self, _rhs: Self) {}
}

impl SubAssign for NoWeight {
    fn sub_assign(&mut self, _rhs: Self) {}
}

impl fmt::Display for NoWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "()")
    }
}

impl fmt::Debug for NoWeight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "()")
    }
}

impl Weight for i8 {}
impl Weight for u8 {}
impl Weight for i16 {}
impl Weight for u16 {}
impl Weight for i32 {}
impl Weight for u32 {}
impl Weight for i64 {}
impl Weight for u64 {}
impl Weight for i128 {}
impl Weight for u128 {}
impl Weight for isize {}
impl Weight for usize {}
impl Weight for f32 {}
impl Weight for f64 {}
impl Weight for NoWeight {}
