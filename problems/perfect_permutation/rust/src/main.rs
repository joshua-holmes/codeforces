use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let num_of_perms: i32 = s.trim().parse()?;

        let mut result = vec![num_of_perms];
        for i in 1..num_of_perms {
            result.push(i);
        }

        println!("{}", result.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
    }

    Ok(())
}
