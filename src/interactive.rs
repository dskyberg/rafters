use anyhow::Result;

#[allow(unused_imports)]
use inquire::validator::StringValidator;

use inquire::{Text, validator::Validation};

use crate::cli::Cli;
use crate::error::Error;

fn get_f32(prompt: &str) -> Result<f32> {
    let validator = |input: &str| {
        if input.parse::<f32>().is_err() {
            Ok(Validation::Invalid("Please provide a valid number.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let status = Text::new(prompt).with_validator(validator).prompt();

    match status {
        Ok(result) => Ok(result.parse::<f32>()?),
        Err(err) => Err(Error::InvalidInput(err.to_string()).into()),
    }
}

pub fn run() -> Result<Cli> {
    let pitch = get_f32("What's the pitch?")? as u32;
    let span = get_f32("What's the span?")?;
    let wall_width = get_f32("What's the wall width?")?;
    let beam_thickness = get_f32("What's the beam thickness?")?;
    let beam_width = get_f32("What's the beam width?")?;
    let overhang = get_f32("What's the overhang?")?;
    let rafter_width = get_f32("What's the rafter width?")?;
    let cli = Cli {
        pitch,
        span,
        wall_width,
        beam_thickness,
        beam_width,
        overhang,
        rafter_width,
    };
    Ok(cli)
}
