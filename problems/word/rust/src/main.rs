use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let word = s.trim();

    let lowercase_count: u8 = word.chars().map(|c| c.is_lowercase() as u8).sum();
    let result = if lowercase_count * 2 >= word.len() as u8 {
        word.to_lowercase()
    } else {
        word.to_uppercase()
    };

    println!("{}", result);

    Ok(())
}
