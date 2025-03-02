pub trait Digits {
    fn digits(&self, num_decimals: u32) -> Self;
}

/// Mathematically round to `num_decimals`.  This is not simply a
/// string trim function.
impl Digits for f32 {
    fn digits(&self, num_decimals: u32) -> Self {
        let y = 10i32.pow(num_decimals) as f32;
        (*self * y).round() / y
    }
}
