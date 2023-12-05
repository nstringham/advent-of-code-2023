use advent_of_code_2023::stdin_lines;

fn max_allowed(color: &str) -> Result<usize, &str> {
    match color {
        "red" => Ok(12),
        "green" => Ok(13),
        "blue" => Ok(14),
        _ => Err("unknown color"),
    }
}

fn main() {
    let sum: usize = stdin_lines()
        .enumerate()
        .filter(|(_i, line)| {
            line.split(':').nth(1).unwrap().split(";").all(|handful| {
                handful.split(',').all(|pair| {
                    let mut pair = pair.trim().split_whitespace();
                    let amount = pair.next().unwrap().parse::<usize>().unwrap();
                    let color = pair.next().unwrap();
                    amount <= max_allowed(color).unwrap()
                })
            })
        })
        .map(|(i, _line)| i + 1)
        .sum();

    println!("{sum}");
}
