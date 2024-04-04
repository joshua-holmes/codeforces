use std::{error::Error, io};

// 4 hours
const CONTEST_LENGTH: i32 = 4 * 60;

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let input: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();
    let n = input[0];
    let k = input[1];

    let time_remaining = CONTEST_LENGTH - k;

    let mut sum = 0;
    for i in 1..=n {
        sum += i * 5;
        if sum > time_remaining {
            println!("{}", i - 1);
            return Ok(())
        }
    }

    println!("{}", n);

    Ok(())
}
