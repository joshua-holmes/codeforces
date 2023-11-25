use std::{error::Error, io, cmp};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut nums_iter = s.trim().split(" ").map(|n| n.parse::<i64>().unwrap());
    let (n, l, r) = (
        nums_iter.next().unwrap(),
        nums_iter.next().unwrap(),
        nums_iter.next().unwrap(),
    );

    let min = {
        let num_of_ones = n - l + 1;
        let num_of_non_ones = l - 1;

        let mut sum = num_of_ones;

        for i in 1..=num_of_non_ones {
            let two: i64 = 2;
            let non_one = two.pow(i as u32);

            sum += non_one;
        }

        sum
    };

    let max = {
        let num_of_ones = if r > 1 { 1 } else { n };
        let num_of_non_ones = n - num_of_ones;

        let mut sum = num_of_ones;

        for i in 1..=num_of_non_ones {
            let pow = cmp::min(i, r - 1);
            let two: i64 = 2;
            let non_one = two.pow(pow as u32);

            sum += non_one;
        }

        sum
    };

    println!("{} {}", min, max);

    Ok(())
}
