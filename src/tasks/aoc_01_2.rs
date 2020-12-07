use std::collections::HashSet;

use anyhow::Result;

use crate::input;
pub fn run() -> Result<()> {
	let entries = input!()?.iter().map(|s| s.parse::<i32>().unwrap()).collect::<HashSet<_>>();

	match entries.iter().find_map(|x| {
		entries.iter().filter(|&y| x != y).find_map(|y| entries.get(&(2020 - x - y)).map(|z| x * y * z))
	}) {
		Some(v) => println!("{}", v),
		None => println!("None found"),
	}
	Ok(())
}