use crate::exercise::ExerciseList;
use colored::Colorize;
use std::fs;

mod exercise;

fn main() {
    let exercises_str = match fs::read_to_string("info.toml") {
        Ok(s) => s,
        Err(_) => {
            println!(
                "{}",
                "rustlings should be run from the (solutions') repository directory!".red()
            );
            std::process::exit(1);
        }
    };

    let exercises = match toml::from_str::<ExerciseList>(&exercises_str) {
        Ok(e) => e.exercises,
        Err(e) => {
            println!(
                "{}",
                format!("Could not parse info.toml: {e}, maybe your file is corrupted?",).red()
            );
            std::process::exit(1);
        }
    };

    for e in exercises {
        if fs::File::open(&e.path).is_err() {
            println!("{}", format!("Exercise {} is missing!", e.name).red());
            std::process::exit(1);
        }
    }

    println!("{}", "Solution to all exercises found.".green());
    println!("Try `{}`. Enjoy it!", "rustlings verify".bold());
}
