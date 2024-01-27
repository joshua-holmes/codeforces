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

        let result = if has_one_zero && is_divisible {
            let mut winners = Vec::with_capacity(n as usize - 1);
            let mut i = 0;
            while i < n - 1 {
                let winner = i + 2;
                for _ in 0..non_zero {
                    winners.push(winner.to_string());
                    i += 1;
                }
            }
            winners.join(" ")
        } else {
            (-1).to_string()
        };

        println!("{}", result);
    }

    Ok(())
}
