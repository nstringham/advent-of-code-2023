use advent_of_code_2023::stdin_lines;

fn is_digit(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9')
}

fn extract_number(row: &mut [u8], index: usize) -> i32 {
    if !is_digit(row[index]) {
        return 0;
    }

    let start = (0..index).rev().find(|&i| !is_digit(row[i])).unwrap_or(0) + 1;

    let end = (index + 1..row.len())
        .find(|&i| !is_digit(row[i]))
        .unwrap_or(0)
        - 1;

    row[start..=end].iter_mut().fold(0, |number, byte| {
        let digit = *byte - b'0';
        *byte = b'.';
        number * 10 + digit as i32
    })
}

fn extract_from_row(row: &mut [u8]) -> i32 {
    (0..row.len())
        .filter_map(|i| match row[i] {
            b'0'..=b'9' => None,
            b'.' => None,
            _ => Some(extract_number(row, i + 1) + extract_number(row, i + 1)),
        })
        .sum()
}

fn extract_from_rows(row1: &mut [u8], row2: &mut [u8]) -> i32 {
    assert_eq!(row1.len(), row2.len());

    let total1: i32 = (0..row1.len())
        .filter_map(|i| match row1[i] {
            b'0'..=b'9' => None,
            b'.' => None,
            _ => Some(
                extract_number(row2, i - 1) + extract_number(row2, i) + extract_number(row2, i + 1),
            ),
        })
        .sum();

    let total2: i32 = (0..row2.len())
        .filter_map(|i| match row2[i] {
            b'0'..=b'9' => None,
            b'.' => None,
            _ => Some(
                extract_number(row1, i - 1) + extract_number(row1, i) + extract_number(row1, i + 1),
            ),
        })
        .sum();

    total1 + total2
}

fn main() {
    let mut rows = stdin_lines().map(|line| line.into_bytes());

    let mut total = 0;

    // println!("{sum}");
}
