use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let buf = BufReader::new(std::io::stdin()).lines().map(Result::unwrap);

	Ok(())
}