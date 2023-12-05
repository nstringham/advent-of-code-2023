use std::io::stdin;

pub fn stdin_lines() -> impl Iterator<Item = String> {
    stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
}
