#![no_std]
#![feature(const_fn)]

use core::ops::*;
use core::fmt::{Formatter, Display, Debug, Error};

const FRACTION_BITS: u8 = 8;
const FRACTION_MASK: i32 = 0xFF;
const FRACTION_VALUE: i32 = 256;
const SIGN_MASK: u32 = 0x8000_0000;
const INT_BITS: u8 = 24;
const BIT_VALUE: f64 = 0.00390625;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct F24P8 {
    bits: i32
}
impl Add for F24P8 {
    type Output = F24P8;
    fn add(self, rhs: Self) -> Self {
        F24P8 { bits: self.bits + rhs.bits }
    }
}
impl Sub for F24P8 {
    type Output = F24P8;
    fn sub(self, rhs: Self) -> Self {
        F24P8 { bits: self.bits - rhs.bits }
    }
}
impl Mul for F24P8 {
    type Output = F24P8;
    fn mul(self, rhs: Self) -> Self {
        F24P8 { bits: (self.bits * rhs.bits) / FRACTION_VALUE }
    }
}
impl Mul<i32> for F24P8 {
    type Output = F24P8;
    fn mul(self, rhs: i32) -> Self {
        F24P8 { bits: self.bits * rhs }
    }
}
impl Div<i32> for F24P8 {
    type Output = F24P8;
    fn div(self, rhs: i32) -> Self {
        F24P8 { bits: self.bits / rhs }
    }
}
impl Display for F24P8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{:08}",
            self.bits >> FRACTION_BITS,
            (self.bits & FRACTION_MASK) * 100_000_000
        )
    }
}
impl Debug for F24P8 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:06x}.{:02x}",
            self.bits >> FRACTION_BITS,
            self.bits & FRACTION_MASK
        )
    }
}
impl From<f64> for F24P8 {
    fn from(f: f64) -> F24P8 {
        assert!(f >= 0.);
        F24P8 { bits: (f * FRACTION_VALUE as f64) as i32 }
    }
}
impl From<i32> for F24P8 {
    fn from(i: i32) -> F24P8 {
        F24P8 { bits: i * FRACTION_VALUE }
    }
}
impl From<F24P8> for f64 {
    fn from(fp: F24P8) -> f64 {
        fp.bits as f64 * BIT_VALUE
    }
}
impl F24P8 {
    pub fn twice(self) -> F24P8 {
        F24P8 { bits: self.bits * 2 }
    }
    pub fn half(self) -> F24P8 {
        F24P8 { bits: self.bits / 2 }
    }
    pub const fn from_int(i: i32) -> F24P8 {
        F24P8 { bits: i * FRACTION_VALUE }
    }
    pub const fn from_float(f: f64) -> F24P8 {
        F24P8 { bits: (f * FRACTION_VALUE as f64) as i32 }
    }
}
