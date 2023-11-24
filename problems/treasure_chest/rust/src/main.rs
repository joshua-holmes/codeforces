use std::{error::Error, io, cmp};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let mut input_iter = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap());
        let (chest, key, max_steps) = (
            input_iter.next().unwrap(),
            input_iter.next().unwrap(),
            input_iter.next().unwrap(),
        );

        if key < chest {
            println!("{}", chest);
        } else if max_steps >= (key - chest) {
            println!("{}", key);
        } else {
            let dist_w_chest = chest + max_steps;
            let dist_wo_chest = cmp::max(key - dist_w_chest, 0);
            let distance = dist_w_chest + (dist_wo_chest * 2);

            println!("{}", distance);
        }
    }

    Ok(())
}
