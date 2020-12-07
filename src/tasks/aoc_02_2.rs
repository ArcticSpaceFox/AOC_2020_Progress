use anyhow::Result;

use crate::input;

use super::aoc_02_1::Password;

impl Password {
	pub fn valid_2(&self) -> bool {
		let test: Vec<char> = self.password.chars().collect();
		(test[self.policy_min - 1] == self.letter)^(test[self.policy_max - 1] == self.letter)
	}
}

pub fn run() -> Result<()> {
	let input = input!()?;
	let passwords = input.iter().map(|x| x.parse::<Password>().unwrap());

	let soloution = passwords.filter(Password::valid_2).count();
	println!("{}", soloution);
	
	Ok(())
}