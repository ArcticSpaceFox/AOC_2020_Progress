use anyhow::Result;

use crate::input;

pub fn ride(data: &Vec<Vec<bool>>, r: usize, d: usize) -> usize {
    let mut offset: usize = 0;
	let mut count: usize = 0;
    
    for line in data.iter().skip(d).step_by(d) {
		offset += r;
		if line[offset % line.len()]  {
			count += 1;
		}
    }

    count
}

pub fn run() -> Result<()> {
    let data = input!()?;
    let data = data
        .iter()
        .map(|e| e.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();


	println!("{}", ride(&data, 3, 1));

    Ok(())
}
