use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let soroban_str: &str = s.trim();
    let nums: Vec<u8> = soroban_str
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    let lines = nums.into_iter().map(|n| {
        let left = if n >=5 {
            "-O"
        } else {
            "O-"
        };
        let right = match n % 5 {
            0 => "-OOOO",
            1 => "O-OOO",
            2 => "OO-OO",
            3 => "OOO-O",
            4 => "OOOO-",
            _ => unreachable!()
        };

        format!("{}|{}", left, right)
    }).rev().collect::<Vec<_>>();

    println!("{}", lines.join("\n"));

    Ok(())
}
