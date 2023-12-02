use std::io::stdin;

struct Bag {
    red: usize,
    blue: usize,
    green: usize,
}

impl Default for Bag {
    fn default() -> Self {
        Bag {
            red: 0,
            blue: 0,
            green: 0,
        }
    }
}

impl Bag {
    fn consider(&mut self, color: &str, amount: usize) {
        match color {
            "red" if amount > self.red => self.red = amount,
            "green" if amount > self.green => self.green = amount,
            "blue" if amount > self.blue => self.blue = amount,
            _ => {}
        }
    }

    fn power(&self) -> usize {
        self.red * self.blue * self.green
    }
}

fn main() {
    let sum: usize = stdin()
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut bag = Bag::default();
            let handfuls = line.split(':').nth(1).unwrap().split(";");
            for pair in handfuls.flat_map(|handful| handful.split(',')) {
                let mut pair = pair.trim().split_whitespace();
                let amount = pair.next().unwrap().parse::<usize>().unwrap();
                let color = pair.next().unwrap();
                bag.consider(color, amount);
            }
            bag.power()
        })
        .sum();

    println!("{sum}");
}
