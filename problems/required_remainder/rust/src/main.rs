use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let input = s.trim().split(" ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let [x, y, n] = match input.as_slice() {
            [x, y, n] => [x, y, n],
            _ => unreachable!()
        };

        let multi = (n - y) / x;
        let answer = (x * multi) + y;
        println!("{}", answer);
    }

    Ok(())
}
