use advent_of_code_2023::stdin_lines;

fn is_digit(byte: u8) -> bool {
    matches!(byte, b'0'..=b'9')
}

fn extract_number(row: &mut [u8], index: usize) -> i32 {
    if !is_digit(row[index]) {
        return 0;
    }

    let start = (0..index)
        .rev()
        .find(|&i| !is_digit(row[i]))
        .map(|i| i + 1)
        .unwrap_or(0);

    let end = (index + 1..row.len())
        .find(|&i| !is_digit(row[i]))
        .unwrap_or(row.len());

    row[start..end].iter_mut().fold(0, |number, byte| {
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
            _ => Some(extract_number(row, i - 1) + extract_number(row, i + 1)),
        })
        .sum()
}

fn extract_from_rows(number_row: &mut [u8], symbol_row: &mut [u8]) -> i32 {
    assert_eq!(symbol_row.len(), number_row.len());

    (0..symbol_row.len())
        .filter_map(|i| match symbol_row[i] {
            b'0'..=b'9' => None,
            b'.' => None,
            _ => Some(
                extract_number(number_row, i - 1)
                    + extract_number(number_row, i)
                    + extract_number(number_row, i + 1),
            ),
        })
        .sum()
}

fn main() {
    let mut rows = stdin_lines().map(|line| line.into_bytes());

    let mut total = 0;

    let mut previous_row = rows.next().unwrap();
    total += extract_from_row(&mut previous_row);

    for mut row in rows {
        total += extract_from_row(&mut row);
        total += extract_from_rows(&mut previous_row, &mut row);
        total += extract_from_rows(&mut row, &mut previous_row);
        previous_row = row;
    }

    println!("{total}");
}
