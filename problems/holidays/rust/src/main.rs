use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let days_per_year: i32 = s.trim().parse()?;

    let weeks_per_year = days_per_year / 7;
    let remainder = days_per_year % 7;

    let min = weeks_per_year * 2 + if remainder == 6 { 1 } else { 0 };
    let max = weeks_per_year * 2 + if remainder == 1 { 1 } else if remainder >= 2 { 2 } else { 0 };

    println!("{} {}", min, max);
 
    Ok(())
}
