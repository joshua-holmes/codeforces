use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let sequence: Vec<i64> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

        let max = sequence.iter().max().unwrap().clone();

        let (min, second_min) = {
            let (mut min, mut second_min) = (i64::MAX, i64::MAX);
            for n in sequence {
                if n <= min {
                    second_min = min;
                    min = n;
                } else if n < second_min {
                    second_min = n;
                }
            }

            (min, second_min)
        };

        let result = if min + second_min < max {
            "YES"
        } else {
            "NO"
        };

        println!("{}", result);
    }

    Ok(())
}
