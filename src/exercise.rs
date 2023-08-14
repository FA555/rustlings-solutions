use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
}
