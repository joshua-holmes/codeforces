use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let reviewers: Vec<&str> = s.trim().split(" ").collect();
        let mut total_likes = 0;
        for r in reviewers {
            match r {
                "1" => total_likes += 1,
                "3" => total_likes += 1,
                _ => {}
            }
        }

        println!("{}", total_likes);
    }

    Ok(())
}
