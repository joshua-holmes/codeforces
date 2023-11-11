use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let nums: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();
        
        let mut diff = 0;
        for (i, n) in nums.iter().enumerate() {
            let j = i as i32 + 1 + diff;
            if *n <= j {
                continue;
            }
            diff += n - j;
        }
        println!("{}", diff);
    }

    Ok(())
}
