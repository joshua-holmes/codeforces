use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let array: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let mut prev_num1 = None;
    let mut prev_num2 = None;
    let mut count = 0;

    for n in array {
        if let (Some(n1), Some(n2)) = (prev_num1, prev_num2) {
            let is_min = n2 < n1 && n2 < n;
            let is_max = n2 > n1 && n2 > n;
            if is_min || is_max {
                count += 1;
            }
        }

        prev_num1 = prev_num2;
        prev_num2 = Some(n);
    }

    println!("{}", count);

    Ok(())
}
