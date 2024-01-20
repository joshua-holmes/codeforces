use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let n: usize = s.trim().parse()?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let s = s.trim();

        let common_num = s.chars().nth(n - 1).unwrap();

        let result = String::from_utf8(vec![common_num as u8; n])?;

        println!("{}", result);
    }

    Ok(())
}
