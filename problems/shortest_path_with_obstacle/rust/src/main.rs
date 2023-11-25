use std::{error::Error, io, cmp};

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn new_from(x: i32, y: i32) -> Self {
        Coordinates { x, y }
    }
}

const NUM_OF_POINTS: usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let num_of_tests: i32 = s.trim().parse()?;

    for _ in 0..num_of_tests {
        io::stdin().read_line(&mut String::new())?;

        let mut points: [Coordinates; NUM_OF_POINTS] = [Coordinates::new_from(0, 0); NUM_OF_POINTS];
        for i in 0..NUM_OF_POINTS {
            let mut s = String::new();
            io::stdin().read_line(&mut s)?;
            let mut point_iter = s.trim().split(" ").map(|n| n.parse::<i32>().unwrap());
            let point = Coordinates::new_from(point_iter.next().unwrap(), point_iter.next().unwrap());
            points[i] = point;
        };
        let [a, b, f] = points;

        let answer = if a.x == b.x && a.x == f.x && f.y > cmp::min(a.y, b.y) && f.y < cmp::max(a.y, b.y) {
            (a.y - b.y).abs() + 2
        } else if a.y == b.y && a.y == f.y && f.x > cmp::min(a.x, b.x) && f.x < cmp::max(a.x, b.x) {
            (a.x - b.x).abs() + 2
        } else {
            (a.x - b.x).abs() + (a.y - b.y).abs()
        };

        println!("{}", answer);
    }

    Ok(())
}
