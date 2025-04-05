#[allow(dead_code)]
/// Format a  decimal length in inches
pub fn to_inches(length: f32) -> String {
    let fract = length.fract();
    let length = length as u32;
    let feet = length / 12;
    let inches = length - (feet * 12);

    let mut result = String::new();
    if feet > 0 {
        result.push_str(&format!("{}'", feet));
    }
    if inches > 0 {
        result.push_str(&format!(" {}", inches));
    }
    if fract > 0.0 {
        let sixteenths = (fract * 16.0) as u32;
        result.push_str(&format!(" {}/16", sixteenths));
    }
    if inches > 0 || fract > 0.0 {
        result.push('\'');
    }
    result
}

#[cfg(test)]
mod tests {
    use rafter_lib::toa;
    use round_to::RoundTo;

    const OPPOSITE: f32 = 3.0;
    const HYPOTENUSE: f32 = 5.0;
    const ADJACENT: f32 = 4.0;
    const PITCH: u32 = 9;

    #[test]
    fn test_toa() {
        let (rise, length) = toa(PITCH, None, Some(ADJACENT));
        assert_eq!(
            (rise.round_to(2), length.round_to(2)),
            (OPPOSITE, HYPOTENUSE)
        );
        let (run, length) = toa(PITCH, Some(OPPOSITE), None);
        assert_eq!(
            (run.round_to(2), length.round_to(2)),
            (ADJACENT, HYPOTENUSE)
        );
    }
}
