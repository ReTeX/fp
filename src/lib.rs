#![no_std]

use core::ops::*;
use core::fmt::{Formatter, Display, Error};

const FRACTION_BITS: u8 = 8;
const FRACTION_MASK: u32 = 0xFF;
const INT_BITS: u8 = 24;

#[derive(Copy, Clone)]
pub struct F24P8 {
    bits: u32
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
        F24P8 { bits: (self.bits * rhs.bits) >> FRACTION_BITS }
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
impl From<f64> for F24P8 {
    fn from(f: f64) -> F24P8 {
        assert!(f >= 0.);
        F24P8 { bits: (f * (1<<FRACTION_BITS) as f64) as u32 }
    }
}
