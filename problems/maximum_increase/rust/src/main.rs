use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let nums: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let mut sub_arr_len = 1;
    let mut max_sub_arr_len = 1;
    let mut prev_n = nums[0];
    for n in &nums[1..] {
        if *n > prev_n {
            sub_arr_len += 1;
            max_sub_arr_len = std::cmp::max(max_sub_arr_len, sub_arr_len);
        } else {
            sub_arr_len = 1;
        }
        prev_n = *n;
    }

    println!("{}", max_sub_arr_len);

    Ok(())
}
