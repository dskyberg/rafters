//! IRC states that the seat cut on a bird's mouth must be min 1.5 inches.  The seat should never be
//! wider than the plate. And the heal cannot span more than 1/4 the width of the rafter.
use clap::Parser;

mod birds_mouth;
mod cli;
mod digits;
mod rafter;
mod right_angle;
mod tail;
mod utils;

use cli::Cli;
use digits::Digits;
use rafter::Rafter;
use right_angle::RightAngle;
use utils::to_inches;

fn main() {
    let cli = Cli::parse();
    let rafter = Rafter::from_cli(&cli);
    println!("The angle of the pitch: {}", rafter.angle().digits(2));
    println!(
        "Rafter length (from the ridge beam to the bird's mouth heel): {}",
        to_inches(rafter.total_length())
    );
    println!("Ridge Height: {}", to_inches(rafter.ridge_beam_height()));
    println!(
        "Tail length (from the tip of the rafter to the wall): {}",
        to_inches(rafter.tail_length())
    );
    println!(
        "Bird's mouth length: {}",
        to_inches(rafter.birds_mouth_length())
    );
}
