use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    'tests: for _ in 0..num_of_tests {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        let students: Vec<Vec<u8>> = (0..s.trim().parse::<i32>()?).map(|_| {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            s.trim().split(" ").map(|n| n.parse().unwrap()).collect()
        })
        .collect();

        let mut days: [u64; 5] = [0; 5];
        for s in students.iter() {
            for i in 0..s.len() {
                if s[i] == 1 {
                    days[i] += 1;
                }
            }
        }

        let mut eligible_days = Vec::new();
        for (i, d) in days.iter().enumerate() {
            if *d >= students.len() as u64 / 2 {
                eligible_days.push(i);
            }
        }

        let len = eligible_days.len();
        if len < 2 {
            println!("NO");
            continue 'tests;
        }

        for i in 0..(len - 1) {
            'day: for j in (i + 1)..len {
                let day1 = eligible_days[i];
                let day2 = eligible_days[j];
                for s in students.iter() {
                    if s[day1] != 1 && s[day2] != 1 {
                        continue 'day;
                    }
                }
                println!("YES");
                continue 'tests;
            }
        }
        println!("NO");
    }

    Ok(())
}
