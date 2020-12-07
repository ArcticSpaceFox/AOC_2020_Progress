use std::str::FromStr;

use anyhow::Result;

use crate::input;

#[derive(Debug)]
pub struct Password {
    pub policy_min: usize,
    pub policy_max: usize,
    pub letter: char,
    pub password: String,
}

impl FromStr for Password {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, password) = s.split_once(": ").unwrap();
        let (rest, letter) = rest.split_once(" ").unwrap();
        let (policy_min, policy_max) = rest.split_once("-").unwrap();

		let policy_min = policy_min.parse().unwrap();
		let policy_max = policy_max.parse().unwrap();
		let letter = letter.chars().next().unwrap();
		let password = password.to_owned();

        Ok(Self {
            policy_min,
            policy_max,
            letter,
            password,
        })
    }
}

impl Password {
    pub fn valid(&self) -> bool {
		let test = self.password.chars().filter(|&c| c == self.letter).count();
		self.policy_min <= test && test <= self.policy_max
    }
}

pub fn run() -> Result<()> {
	let input = input!()?;
	let passwords = input.iter().map(|x| x.parse::<Password>().unwrap());

	let soloution = passwords.filter(Password::valid).count();
	println!("{}", soloution);

    Ok(())
}
