# fp
Fixed Point Types

Currently only implements F24P8. (i32 / 256).
Supported operations are Add, AddAssign, Sub, SubAssign, and Mul.
Conversions from (u8, i8, u16, i16, u32, i32) -> F24P8 and f64 -> F24P8 -> f64 are implemented.

Scalar multiplication: Int * F24P8 and F24P8 / Int.
