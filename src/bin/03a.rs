use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use advent_of_code_2023::stdin_lines;

#[derive(Debug)]
enum Cell {
    Symbol(usize),
    Digit(usize, usize, usize),
}

struct CellParser<'a> {
    inner: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> CellParser<'a> {
    pub fn parse(string: &'a str) -> Self {
        Self {
            inner: string.chars().enumerate().peekable(),
        }
    }
}

impl<'a> Iterator for CellParser<'a> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (index, char) = self.inner.next()?;
            match char {
                '.' => continue,
                '0'..='9' => {
                    let mut number = char as usize - '0' as usize;
                    let mut ending_index = index;
                    while matches!(self.inner.peek(), Some((_, '0'..='9'))) {
                        let (index, char) = self.inner.next().unwrap();
                        number = number * 10 + char as usize - '0' as usize;
                        ending_index = index
                    }
                    break Some(Cell::Digit(index, ending_index, number));
                }
                _ => break Some(Cell::Symbol(index)),
            };
        }
    }
}

fn main() {
    let lines: Vec<Vec<Cell>> = stdin_lines()
        .map(|line| CellParser::parse(&line).collect())
        .collect();

    let sum: usize = lines
        .into_iter()
        .flat_map(|line| line.into_iter())
        .filter_map(|cell| match cell {
            Cell::Digit(_, _, number) => Some(number),
            _ => None,
        })
        .sum();

    println!("{sum}");
}
