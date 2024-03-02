use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let cream: Vec<i32> = s.trim().split(" ").map(|c| c.parse().unwrap()).collect();

        let mut layers = Vec::with_capacity(cream.len());

        let mut first_dry_layer: i32 = 0;
        for (i, c) in cream.into_iter().rev().enumerate() {
            first_dry_layer = std::cmp::max(c + i as i32, first_dry_layer);
            let wet_layers_left = first_dry_layer - i as i32;
            let is_wet = wet_layers_left > 0;
            if c > wet_layers_left {
                first_dry_layer = c + i as i32;
            }
            layers.push(if is_wet { 1 } else { 0 });
        }

        println!("{}", layers.into_iter().rev().map(|n| n.to_string()).collect::<Vec<_>>().join(" "))
    }

    Ok(())
}
