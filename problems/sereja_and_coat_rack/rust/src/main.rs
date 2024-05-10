use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let input: Vec<i32> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();
    let [_, d] = [input[0], input[1]];

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let mut hook_prices: Vec<i32> = s
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>();
    hook_prices.sort();

    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let guests: i32 = s.trim().parse()?;

    let revenue = &hook_prices[..std::cmp::min(guests as usize, hook_prices.len()) as usize]
        .iter()
        .sum::<i32>();

    let remaining_guests = std::cmp::max(0, guests - hook_prices.len() as i32);

    let profit = revenue - remaining_guests * d;

    println!("{}", profit);

    Ok(())
}
