use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin().lock();

    let lines = stdin
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty());

    let sum: i32 = lines
        .map(|line| {
            let first = line.chars().find(|c| matches!(c, '0'..='9')).unwrap();
            let last = line.chars().rev().find(|c| matches!(c, '0'..='9')).unwrap();
            (first as i32 - '0' as i32) * 10 + (last as i32 - '0' as i32)
        })
        .sum();

    println!("{sum}");
}
