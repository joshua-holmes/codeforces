use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let ns: Vec<i32> = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect();

    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let ms: Vec<i32> = s.trim().split(" ").map(|m| m.parse::<i32>().unwrap()).collect();

    let mut max_int = -1;
    let mut max_int_count = 0;
    for n in ns.iter() {
        for m in ms.iter() {
            if m % n != 0 {
                continue;
            }

            let integer = m / n;
            if integer > max_int {
                max_int = integer;
                max_int_count = 1;
            } else if integer == max_int {
                max_int_count += 1;
            }
        }
    }

    println!("{}", max_int_count);

    Ok(())
}
