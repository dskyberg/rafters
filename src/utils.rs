//! This utility library is partly in support of the rafters project and partly in support
//! of simply learning programmatic trigonometry.  The `dead_code` methods are just for trig fun.

#[allow(dead_code)]
pub const DEGREE_SYMBOL: char = '°';

/// Calculate the angle is for a given pitch.
/// Given pitch is just the rise, when the run is 12, calculate the angle as
/// `tan(θ) = pitch/12`
#[inline(always)]
pub fn angle_from_pitch(pitch: u32) -> f32 {
    (pitch as f32 / 12.0).atan().to_degrees()
}

#[allow(dead_code)]
/// Calculate the pitch for a given angle in degrees.
/// Given angle is the angle of the triangle.  The run is always 12.
/// Calculate the pitch as `pitch = tan(θ) * 12`
#[inline(always)]
pub fn pitch_from_angle(angle: f32) -> u32 {
    (angle.to_radians().tan() * 12.0) as u32
}

/// Given an angle and the run, calculate the rise and length
pub fn calculate_rise_and_length(pitch: u32, run: f32) -> (f32, f32) {
    let rad = (pitch as f32 / 12.0).atan();
    let rise = rad.tan() * run;
    let hypotenuse = (rise.powi(2) + run.powi(2)).sqrt();
    (rise, hypotenuse)
}

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
    use super::*;
    use crate::digits::Digits;

    const OPPOSITE: f32 = 3.0;
    const HYPOTENUSE: f32 = 5.0;
    const ADJACENT: f32 = 4.0;
    const ANGLE: f32 = 36.87; // Radians = 0.6435
    const PITCH: u32 = 9;

    #[test]
    fn test_calculate_rise_and_length() {
        let (rise, length) = calculate_rise_and_length(PITCH, ADJACENT);
        assert_eq!((rise.digits(2), length.digits(2)), (OPPOSITE, HYPOTENUSE));
    }

    #[test]
    fn test_angle_from_pitch() {
        let angle = angle_from_pitch(PITCH);
        assert_eq!(angle.digits(2), ANGLE);
    }

    #[test]
    fn test_pitch_from_angle() {
        let pitch = pitch_from_angle(ANGLE);
        assert_eq!(pitch, PITCH);
    }
}
