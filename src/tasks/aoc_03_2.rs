use anyhow::Result;

use super::aoc_03_1::ride;
use crate::input;

pub fn run() -> Result<()> {
    let data = input!()?;
    let data = data
        .iter()
        .map(|e| e.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let moves = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut count = 1usize;

    for (right, down) in moves {
        count *= ride(&data, right, down);
    }

    println!("{}", count);

    Ok(())
}
