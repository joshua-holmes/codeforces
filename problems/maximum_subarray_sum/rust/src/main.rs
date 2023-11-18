use std::{error::Error, io, cmp};

fn main() -> Result<(), Box<dyn Error>> {
    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let nums: Vec<i64> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let mut max_sum = i64::MIN;
    let mut cur_sum: i64 = 0;
    for n in nums {
        cur_sum += n;
        max_sum = cmp::max(max_sum, cur_sum);
        if cur_sum < 0 {
            cur_sum = 0;
        }
    }

    println!("{}", max_sum);

    Ok(())
}
