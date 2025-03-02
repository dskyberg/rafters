use crate::birds_mouth::BirdsMouth;
use crate::cli::Cli;
use crate::digits::Digits;
use crate::right_angle::RightAngle;
use crate::tail::Tail;
use crate::utils::*;

pub struct Rafter {
    width: f32,
    rise: f32,
    run: f32,
    length: f32,
    pitch: u32,
    pub birds_mouth: BirdsMouth,
    pub tail: Tail,
}

impl RightAngle<f32> for Rafter {
    fn rise(&self) -> f32 {
        self.rise
    }
    fn run(&self) -> f32 {
        self.run
    }
    fn length(&self) -> f32 {
        self.length
    }
    fn angle(&self) -> f32 {
        angle_from_pitch(self.pitch)
    }
}

/// The run of the rafter is calculated from the outside of the wall (including sheathing) to
/// the center of the structure, minuse 1/2 the thickness of the ridge board/beam.
///
/// The ridge beam height is then the rafter rise - the bird's mouth rise.
///
/// And finally, the total length is the tail length + the rafter length
impl Rafter {
    pub fn from_cli(cli: &Cli) -> Self {
        // First calculate the big triangle
        let run = (cli.span / 2.0) - (cli.beam_thickness / 2.0);
        let (rise, length) = calculate_rise_and_length(cli.pitch, run);

        // The bird's mouth is pretty standard.  We just need to adjust for the
        // width of the rafter, and wall to ensure it meets code.
        let birds_mouth = BirdsMouth::from_cli(cli);
        let tail = Tail::from_pitch_and_run(cli.pitch, cli.overhang);
        // Calculate the angled rafter width, so that we can calculate the ridge beam height
        let (_, angled_rafter_width) = calculate_rise_and_length(cli.pitch, cli.rafter_width);
        let ridge_beam_height = rise + angled_rafter_width - birds_mouth.rise();

        Rafter {
            width: cli.rafter_width,
            rise: ridge_beam_height,
            run,
            length,
            pitch: cli.pitch,
            birds_mouth,
            tail,
        }
    }

    /// The shear width is the angled width of the rafter
    fn angled_width(&self) -> f32 {
        let cos = self.angle().to_radians().cos();
        (cos * self.width).digits(2)
    }

    pub fn tail_length(&self) -> f32 {
        self.tail.length()
    }

    pub fn birds_mouth_length(&self) -> f32 {
        self.birds_mouth.length()
    }

    pub fn total_length(&self) -> f32 {
        self.tail.length() + self.length
    }

    /// The rafter rise calculates the height of the bottom of the rafter.
    /// So, add the angled_width to get the true rise.  Then we need to
    /// remove the bird's mouth from that to get the actual height.
    pub fn ridge_beam_height(&self) -> f32 {
        self.rise + self.angled_width() - self.birds_mouth.rise()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rafter() {
        let cli = Cli {
            wall_width: 5.5,
            overhang: 18.0,
            pitch: 4,
            beam_thickness: 1.5, // 2x8 board
            rafter_width: 9.25,
            span: 360.0, // 30ft
        };
        let rafter = Rafter::from_cli(&cli);
        assert_eq!(rafter.total_length().digits(2), 207.92);
        assert_eq!(rafter.ridge_beam_height().digits(2), 74.61);
        assert_eq!(rafter.tail_length().digits(2), 18.97);
        assert_eq!(rafter.birds_mouth.run().digits(2), 5.50);
        assert_eq!(rafter.birds_mouth.rise().digits(2), 1.83);
        assert_eq!(rafter.birds_mouth_length().digits(2), 5.80);
    }
}
