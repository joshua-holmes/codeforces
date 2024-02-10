use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let array: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

        let mut going_up = true;
        let mut prev_n = i32::MIN;
        let mut result = "YES";
        for n in array {
            if going_up {
                if n < prev_n {
                    going_up = false;
                }
            } else {
                if n > prev_n {
                    result = "NO";
                    break;
                }
            }

            prev_n = n;
        }

        println!("{}", result);
    }

    Ok(())
}
