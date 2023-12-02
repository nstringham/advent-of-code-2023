use std::io::stdin;

fn main() {
    let lines = stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty());

    let sum: i32 = lines
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            (first as i32 - '0' as i32) * 10 + (last as i32 - '0' as i32)
        })
        .sum();

    println!("{sum}");
}
