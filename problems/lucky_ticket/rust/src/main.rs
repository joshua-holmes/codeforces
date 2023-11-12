use std::io;

fn main() -> io::Result<()> {
    io::stdin().read_line(&mut String::new())?;

    let mut ticket = String::new();
    io::stdin().read_line(&mut ticket)?;
    let ticket = ticket.trim();

    let len = ticket.len();

    let ticket_bytes = ticket.as_bytes();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for (i, b) in ticket_bytes.iter().enumerate() {
        let num: u32 = (*b as char).to_digit(10).unwrap();
        if (num != 4) && (num != 7) {
            println!("NO");
            return Ok(());
        }
        if i < len / 2 {
            sum1 += num;
        } else {
            sum2 += num;
        }
    }

    let output = if sum1 == sum2 { "YES" } else { "NO" };
    println!("{}", output);

    Ok(())
}
