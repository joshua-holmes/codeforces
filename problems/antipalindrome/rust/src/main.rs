use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let word = s.trim().to_string();

    for i in 0..word.len() {
        if !is_palindrome(&word[i..]) {
            println!("{}", word.len() - i);
            return Ok(());
        }
    }

    println!("{}", 0);

    Ok(())
}

fn is_palindrome(word: &str) -> bool {
    let mid = word.len() / 2;
    for i in 0..=mid {
        if word.chars().nth(i).unwrap() != word.chars().nth(word.len() - 1 - i).unwrap() {
            return false;
        }
    }
    true
}
