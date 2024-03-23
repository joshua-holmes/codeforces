use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let flint_guess: i32 = s.trim().parse()?;

        let mut lowest_nearly_primes = [6, 10, 14];
        let mut sum_of_lowest_nearly_primes = lowest_nearly_primes.iter().sum();
        let forth_lowest_nearly_prime = 15;

        let missing_number = flint_guess - sum_of_lowest_nearly_primes;
        if lowest_nearly_primes.contains(&missing_number) {
            lowest_nearly_primes[2] = forth_lowest_nearly_prime;
            sum_of_lowest_nearly_primes = lowest_nearly_primes.iter().sum();
        }

        if flint_guess > sum_of_lowest_nearly_primes {
            let missing_number = flint_guess - sum_of_lowest_nearly_primes;
            let mut answer = lowest_nearly_primes.to_vec();
            answer.push(missing_number);
            println!("YES");
            println!("{}", answer.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
        } else {
            println!("NO");
        }
    }

    Ok(())
}
