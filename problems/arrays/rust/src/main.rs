use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    io::stdin().read_line(&mut String::new())?;

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut input = s.trim().split(" ").map(|n| n.parse::<usize>().unwrap());
    let (k, m) = (input.next().unwrap(), input.next().unwrap());

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let array_a: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut array_b: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();
    array_b.reverse();

    let largest_a = array_a[k - 1];
    let smallest_b = array_b[m - 1];

    let result = if largest_a < smallest_b {
        "YES"
    } else {
        "NO"
    };

    println!("{}", result);

    Ok(())
}
