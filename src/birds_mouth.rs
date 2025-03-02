use crate::cli::Cli;
use crate::right_angle::RightAngle;
use crate::utils::*;

#[derive(Clone)]
pub struct BirdsMouth {
    rise: f32,
    run: f32,
    length: f32,
    angle: f32,
}

impl BirdsMouth {
    pub fn from_cli(cli: &Cli) -> Self {
        Self::check_code(cli)
    }

    /// Ensure that the birds mouth heel is no more than 25% of rafter height,
    /// and the seat is not greater than the top plate width.  Adjust the
    /// bird's mouth accordingly
    fn check_code(cli: &Cli) -> Self {
        let _max_heal = cli.rafter_width / 4.0;
        let max_seat = cli.wall_width;

        let (rise, length) = calculate_rise_and_length(cli.pitch, max_seat);
        if rise > max_seat {
            panic!("Birds mouth seat is greater than top plate width");
        }
        Self {
            rise,
            run: max_seat,
            length,
            angle: angle_from_pitch(cli.pitch),
        }
    }
}
impl RightAngle<f32> for BirdsMouth {
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
        self.angle
    }
}
