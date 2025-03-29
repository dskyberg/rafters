#![doc = include_str!("../README.md")]
use anyhow::Result;
use decimals::Decimals;

mod cli;

mod error;
mod interactive;
mod utils;

use crate::utils::to_inches;
use rafter_lib::{Rafter, RafterInput, RightAngleLike};

use cli::Cli;

fn process_cli(cli: &Cli) -> Result<()> {
    let input: RafterInput = cli.into();
    let rafter = Rafter::from_input(&input);
    let angle = rafter.angle();
    println!("The angle of the pitch: {}", angle.round_to(2));
    println!(
        "Rafter length (from the ridge beam to the bird's mouth heel): {}",
        to_inches(rafter.total_length)
    );
    println!("Ridge Height: {}", to_inches(rafter.ridge_beam_height));
    println!(
        "Tail length (from the tip of the rafter to the wall): {}",
        to_inches(rafter.tail.length())
    );
    println!(
        "Bird's mouth length: {}",
        to_inches(rafter.birds_mouth.length())
    );
    Ok(())
}

fn main() -> Result<()> {
    let cli = match std::env::args().len() < 2 {
        true => interactive::run()?,
        false => cli::run()?,
    };
    process_cli(&cli)
}
