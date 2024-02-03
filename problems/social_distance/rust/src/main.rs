use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let mut n_m_iter = s.trim().split(" ").map(|n| n.parse::<i64>().unwrap());
        let (n, m) = (n_m_iter.next().unwrap(), n_m_iter.next().unwrap());

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let array: Vec<i64> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

        let max = *array.iter().max().unwrap();
        let min = *array.iter().min().unwrap();

        let empty_chairs = array.iter().sum::<i64>() + max - min;

        let required_chairs = empty_chairs + n;

        let answer = if m >= required_chairs {
            "YES"
        } else {
            "NO"
        };

        println!("{}", answer);
    }

    Ok(())
}
