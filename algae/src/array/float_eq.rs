pub trait FloatEq {
    fn zero_threshold() -> Self;
    fn float_eq(&self, other:&Self) -> bool;
}

impl FloatEq for f32 {
    fn zero_threshold() -> Self {
        0.000001
    }

    fn float_eq(&self, other:&Self) -> bool {
        f32::abs(self - other) < Self::zero_threshold()
    }
}

impl FloatEq for f64 {
    fn zero_threshold() -> Self {
        0.000000000000001
    }

    fn float_eq(&self, other:&Self) -> bool {
        f64::abs(self - other) < Self::zero_threshold()
    }
}