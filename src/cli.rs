use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The rise in the rise over run calculation. Such as the 4 in 4:12
    #[arg(short, long, value_name = "4, 7, etc")]
    pub pitch: u32,

    /// Distance in inches between the outside edges of the opposing walls that will hold the rafters.
    /// Remember to include the thickness of sheathing.
    #[arg(short, long, value_name = "372.25 etc")]
    pub span: f32,

    /// Width of the wall (top plate width + sheathing thickness) in inches, such as 5.5 for a 2x6 plate.
    /// This is needed to properly calculate the bird's mouth
    #[arg(short, long, value_name = "6.125, etc")]
    pub wall_width: f32,

    /// Thickness of the ridge board or beam in inches. Such as 1.5 for a typical 2x8 ridge board.
    #[arg(short = 't', long, value_name = "1.5, etc")]
    pub beam_thickness: f32,

    /// Width of the ridge board or beam in inches. Such as 11.25 for a typical 6x12 ridge board.
    #[arg(short, long, value_name = "1.5, etc")]
    pub beam_width: f32,

    /// Distance from the tip of the rafter to the outside edge of the wall in inches.
    #[arg(short, long, value_name = "18.0, 24.0, etc")]
    pub overhang: f32,

    /// Width of the rafter in inches, such as 9.25 for a 2x10 rafter
    #[arg(short, long, value_name = "9.25, etc")]
    pub rafter_width: f32,
}

use rafter_lib::RafterInput;

impl From<&Cli> for RafterInput {
    fn from(input: &Cli) -> Self {
        Self {
            pitch: input.pitch,
            span: input.span,
            wall_width: input.wall_width,
            beam_thickness: input.beam_thickness,
            beam_width: input.beam_width,
            overhang: input.overhang,
            rafter_width: input.rafter_width,
        }
    }
}

pub fn run() -> Result<Cli> {
    let cli = Cli::parse();
    Ok(cli)
}
