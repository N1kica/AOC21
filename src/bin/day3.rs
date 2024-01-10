fn part_one() -> u32 {
    let contents = aoc::chars_per_line("./data/day3.txt", |c| char::to_digit(c, 2));
    let half = (contents.len() / 2) as u32;

    (0..contents[0].len()).map(|i| contents
        .iter()
        .map(|row| row[i])
        .sum::<u32>()
    ).map(|sum| (sum > half) as u32).fold(Diagnostic::new(), |acc, col| Diagnostic {
        gamma: acc.gamma << 1 | col,
        epsilon: acc.epsilon << 1 | col ^ 1,
    }).generate_report()
}

fn main() {
    println!("Part 1: {}", part_one());
}

struct Diagnostic {
    gamma: u32,
    epsilon: u32,
}

impl Diagnostic {
    fn new() -> Self {
        Diagnostic { gamma: 0, epsilon: 0 }
    }

    fn generate_report(&self) -> u32 {
        self.gamma * self.epsilon
    }
}
