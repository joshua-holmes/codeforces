use std::{error::Error, io, collections::HashSet};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_leaves: u8 = s.trim().parse()?;

    let mut collected = HashSet::with_capacity(num_of_leaves as usize);
    for _ in 0..num_of_leaves {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let leaf: &str = s.trim();

        collected.insert(leaf.to_string());
    }

    println!("{}", collected.len());

    Ok(())
}
