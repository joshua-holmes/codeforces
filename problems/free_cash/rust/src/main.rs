use std::{collections::HashMap, error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_visitors: i32 = s.trim().parse()?;

    let mut times: HashMap<String, i32> = HashMap::with_capacity(num_of_visitors as usize);
    for _ in 0..num_of_visitors {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let time = s.trim();

        times
            .entry(time.to_string())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let max = times.values().max().unwrap();

    println!("{}", max);

    Ok(())
}
