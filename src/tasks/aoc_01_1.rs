use std::collections::HashSet;

use anyhow::Result;

use crate::input;

pub fn run() -> Result<()> {
	let buf = input!()?;
	let entries = buf.iter().map(|e| e.parse::<u32>().unwrap()).collect::<HashSet<_>>();

    match entries
        .iter()
        .find_map(|x| entries.get(&(2020 - x)).map(|y| x * y))
    {
        Some(v) => println!("{}", v),
        None => println!("None found"),
    }
	
	Ok(())
}
