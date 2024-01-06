use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let size: i64 = s.trim().parse()?;

    let mut row = (0..size).map(|_| 1).collect::<Vec<i64>>();
    for i in 1..size {
        let mut new_row = Vec::with_capacity((size - i) as usize);
        let mut last_num = row[1];
        for n in &row[1 as usize..] {
            let num = n + last_num;
            new_row.push(num);
            last_num = num;
        }
        row = new_row;
    }

    println!("{}", row[0]);

    Ok(())
}
