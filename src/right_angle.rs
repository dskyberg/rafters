#![allow(dead_code)]

#[inline(always)]
pub fn theorem(a: f32, b: f32) -> f32 {
    (a.powi(2) + b.powi(2)).sqrt()
}

pub trait RightAngle<T: Into<f32> + From<f32>> {
    fn rise(&self) -> T;
    fn run(&self) -> T;
    fn length(&self) -> T;
    fn angle(&self) -> T;

    #[inline(always)]
    fn asin(&self) -> T {
        (self.rise().into() / self.length().into()).asin().into()
    }

    #[inline(always)]
    fn acos(&self) -> T {
        (self.run().into() / self.length().into()).acos().into()
    }

    #[inline(always)]
    fn atan(&self) -> T {
        (self.rise().into() / self.run().into()).atan().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Digits;

    const THETA: f32 = 0.6435;
    struct RightAngleImpl<T: Into<f32> + From<f32> + Copy> {
        rise: T,
        run: T,
        length: T,
    }

    impl<T: Into<f32> + From<f32> + Copy> RightAngle<T> for RightAngleImpl<T> {
        fn rise(&self) -> T {
            self.rise
        }

        fn run(&self) -> T {
            self.run
        }

        fn length(&self) -> T {
            self.length
        }

        fn angle(&self) -> T {
            self.asin()
        }
    }

    const RA: RightAngleImpl<f32> = RightAngleImpl {
        rise: 3.0,
        run: 4.0,
        length: 5.0,
    };

    #[test]
    fn test_theorem() {
        assert_eq!(theorem(RA.rise(), RA.run()), RA.length())
    }

    #[test]
    fn test_equality() {
        let result = vec![THETA; 3];

        assert_eq!(
            result,
            vec![
                RA.asin().digits(4),
                RA.acos().digits(4),
                RA.atan().digits(4)
            ]
        );
    }
    #[test]
    fn test_asin() {
        let theta = RA.asin().digits(4);
        assert_eq!(theta, THETA);
    }

    #[test]
    fn test_cos() {
        let theta = RA.acos().digits(4);
        assert_eq!(theta, THETA);
    }

    #[test]
    fn test_tan() {
        let theta = RA.atan().digits(4);
        assert_eq!(theta, THETA);
    }
}
