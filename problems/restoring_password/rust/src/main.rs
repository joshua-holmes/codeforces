use std::{error::Error, io, collections::HashMap};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let encrypted_pass = s.trim();

    let mut lookup = HashMap::new();
    for i in 0..10 {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let code = s.trim();
        lookup.insert(code.to_string(), i);
    }

    let mut pass = "".to_string();

    for i in 0..8 {
        let chunk = &encrypted_pass[i * 10..(i * 10) + 10];
        let digit = lookup.get(chunk).unwrap();
        pass.push_str(digit.to_string().as_str());
    }

    println!("{pass}");

    Ok(())
}
