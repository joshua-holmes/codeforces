use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let grid_size: i32 = s.trim().parse()?;
        let answer = (grid_size / 2) + 1;
        println!("{}", answer);
    }

    Ok(())
}
