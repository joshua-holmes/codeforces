use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        // num of friends
        let n: i64 = s.trim().parse()?;

        let slices = if n >= 6 {
            n + n % 2
        } else {
            6
        };

        let time = slices * 5 / 2;
        println!("{}", time);
    }

    Ok(())
}
