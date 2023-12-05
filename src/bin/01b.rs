use advent_of_code_2023::stdin_lines;

const NUMBERS: [(&str, i32); 10] = [
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let sum: i32 = stdin_lines()
        .map(|line| {
            let first = line
                .char_indices()
                .filter_map(|(i, c)| {
                    if c.is_ascii_digit() {
                        return Some(c as i32 - '0' as i32);
                    }
                    for (word, int) in NUMBERS {
                        if line[i..].starts_with(word) {
                            return Some(int);
                        }
                    }
                    None
                })
                .nth(0)
                .unwrap();

            let last = line
                .char_indices()
                .rev()
                .filter_map(|(i, c)| {
                    if c.is_ascii_digit() {
                        return Some(c as i32 - '0' as i32);
                    }
                    for (word, int) in NUMBERS {
                        if line[..=i].ends_with(word) {
                            return Some(int);
                        }
                    }
                    None
                })
                .nth(0)
                .unwrap();

            first * 10 + last
        })
        .sum();

    println!("{sum}");
}
