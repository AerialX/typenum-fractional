pub trait Float {
    const F32: f32;
    const F64: f64;

    fn to_f32() -> f32 { Self::F32 }
    fn to_f64() -> f64 { Self::F64 }
}
