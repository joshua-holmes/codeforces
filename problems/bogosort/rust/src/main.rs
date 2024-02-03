use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let mut array: Vec<i64> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

        array.sort();

        let answer = array
            .into_iter()
            .rev()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", answer);
}

    Ok(())
}
