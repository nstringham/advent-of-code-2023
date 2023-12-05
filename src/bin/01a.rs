use advent_of_code_2023::stdin_lines;

fn main() {
    let sum: i32 = stdin_lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            (first as i32 - '0' as i32) * 10 + (last as i32 - '0' as i32)
        })
        .sum();

    println!("{sum}");
}
