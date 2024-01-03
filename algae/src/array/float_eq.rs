pub trait FloatEq {
    fn float_eq(&self, other:&Self) -> bool;
}

use std::f32::EPSILON as F32EPSILON;
use std::f64::EPSILON as F64EPSILON;

// As epsilon is the smallest representable number 
// I might be able to treat it "like" the natural one for floats.
// If every floatingpoint is representable as the product of a natural number and epsilon,
// that would hold.

// 10^-[ 1111 ] {15} * [ .0001 ] { 0.0625 } = 0.0000000000000000625 | 1 * eps
// 10^-[ 1111 ] {15} * [ .0010 ] { 0.125 }  = 0.0000000000000001250 | 2 * eps
// 10^-[ 1111 ] {15} * [ .0011 ] { 0.1875 } = 0.0000000000000001875 | 3 * eps
// 10^-[ 1111 ] {15} * [ .0100 ] { 0.25 }   = 0.0000000000000002500 | 4 * eps
// 10^-[ 1111 ] {15} * [ .1000 ] { 0.5 }    = 0.0000000000000005000 | 8 * eps

// 10^-[ 1111 ] {15} * [ .1010 ] { 0.625 }  = 0.0000000000000006250 | 10 * eps
// 10^-[ 1110 ] {14} * [ .0001 ] { 0.0625 } = 0.0000000000000006250 | 10 * eps
// 10^-[ 1111 ] {15} * [ .1011 ] { 0.6785 } = 0.0000000000000006785 | 11 * eps
// 10^-[ 1110 ] {14} * [ .0010 ] { 0.125 }  = 0.0000000000000012500

// If a shift in the exponenent is equivalent as adding n epsilons this holds.

const THRESHOLD_FACTOR:usize = 5;

impl FloatEq for f32 {
    fn float_eq(&self, other:&Self) -> bool {
        f32::abs(self - other) < F32EPSILON * THRESHOLD_FACTOR as f32
    }
}

impl FloatEq for f64 {
    fn float_eq(&self, other:&Self) -> bool {
        f64::abs(self - other) < F64EPSILON * THRESHOLD_FACTOR as f64
    }
}