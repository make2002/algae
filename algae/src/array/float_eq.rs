pub trait FloatEq {
    fn float_eq(&self, other:&Self) -> bool;
}

impl FloatEq for f32 {
    fn float_eq(&self, other:&Self) -> bool {
        f32::abs(self - other) < 0.000001
    }
}

impl FloatEq for f64 {
    fn float_eq(&self, other:&Self) -> bool {
        f64::abs(self - other) < 0.000000000000001
    }
}