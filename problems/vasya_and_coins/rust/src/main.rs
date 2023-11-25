use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let mut coins_iter = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap());
        let (a, b) = (coins_iter.next().unwrap(), coins_iter.next().unwrap());

        let answer = if a == 0 {
            1
        } else {
            a + (b * 2) + 1
        };

        println!("{}", answer);
    }

    Ok(())
}
