use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::Result;

#[macro_export]
macro_rules! input {
    () => {
        crate::util::input_for_file(file!())
    };
}

pub fn read_input(file: impl AsRef<Path>) -> Result<Vec<String>> {
    let mut path = PathBuf::new();
    path.push("data");
    path.push(file);
    Ok(BufReader::new(File::open(path)?)
        .lines()
        // no idea why but this turbofish is needed
        .collect::<Result<_, _>>()?)
}

pub fn input_for_file(file: &str) -> Result<Vec<String>> {
    let mut buf = PathBuf::new();
    // this works as long as files follow the naming scheme aoc_XX
    buf.push(format!(
        "{}.txt",
        &Path::new(file).file_name().unwrap().to_str().unwrap()[..6]
    ));
    read_input(buf)
}
