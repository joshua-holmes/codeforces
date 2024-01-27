use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let mut nums_iter = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap());
        let (n, x, y) = (
            nums_iter.next().unwrap(),
            nums_iter.next().unwrap(),
            nums_iter.next().unwrap(),
        );

        let non_zero = if x == 0 { y } else { x };
        let has_one_zero = [x, y].into_iter().filter(|n| *n != 0).count() == 1;
        let is_divisible = if non_zero == 0 { false } else { (n - 1) % non_zero == 0 };

            // either x or y must be non-zero
        let result = if has_one_zero && is_divisible {
            (0..(n - 1))
                .map(|i| {
                    let winner = if i < non_zero {
                        1
                    } else {
                        let section = i / non_zero;
                        section * non_zero + 2
                    };
                    winner.to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            (-1).to_string()
        };

        println!("{}", result);
    }

    Ok(())
}
