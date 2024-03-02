use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut iter = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap());
    iter.next();
    let k = iter.next().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut walks: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

    let mut total_walks_added = 0;
    if walks.len() >= 2 {
        for i in 1..walks.len() {
            let prev = walks[i - 1];
            let cur = walks[i];
            let sum = prev + cur;
            let walks_needed = std::cmp::max(k - sum, 0);

            walks[i] += walks_needed;
            total_walks_added += walks_needed;
        }
    }

    let walks_str = walks.into_iter().map(|w| w.to_string()).collect::<Vec<_>>().join(" ");
    println!("{}\n{}", total_walks_added, walks_str);

    Ok(())
}
