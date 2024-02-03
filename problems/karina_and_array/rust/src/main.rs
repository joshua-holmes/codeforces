use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let array: Vec<i64> = s.trim().split(" ").map(|n| n.parse().unwrap()).collect();

        let smallest = *array.iter().min().unwrap();
        let largest = *array.iter().max().unwrap();

        let mut second_smallest = i64::MAX;
        let mut second_largest = i64::MIN;

        let mut found_smallest = false;
        let mut found_largest = false;
        for n in array.iter() {
            if found_smallest || *n != smallest {
                second_smallest = std::cmp::min(*n, second_smallest);
            }
            if found_largest || *n != largest {
                second_largest = std::cmp::max(*n, second_largest);
            }

            if smallest == *n {
                found_smallest = true;
            }
            if largest == *n {
                found_largest = true;
            }
        }

        let answer = std::cmp::max(smallest * second_smallest, second_largest * largest);
        println!("{}", answer);
    }

    Ok(())
}
