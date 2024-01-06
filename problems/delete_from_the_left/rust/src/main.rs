use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let s = buf.trim();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    let t = buf.trim();

    let mut counter = 0;
    for (s, t) in s.chars().rev().zip(t.chars().rev()) {
        if s != t {
            break; 
        }
        counter += 1;
    }

    let result = s.len() as i32 + t.len() as i32 - (counter * 2);
    println!("{}", result);

    Ok(())
}
