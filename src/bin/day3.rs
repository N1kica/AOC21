fn get_diagnostic_report() -> u32 {
    let contents = aoc::chars_per_line("./data/day3.txt", |c| char::to_digit(c, 2));
    let half = (contents.len() / 2) as u32;

    (0..contents[0].len()).map(|i| contents
        .iter()
        .map(|row| row[i])
        .sum::<u32>()
    ).fold(Diagnostic::new(), |acc, col| {
        if col > half {
            Diagnostic {
                gamma: (acc.gamma << 1) | 1,
                epsilon: acc.epsilon << 1,
            }
        } else {
            Diagnostic {
                gamma: acc.gamma << 1,
                epsilon: (acc.epsilon << 1) | 1,
            }
        }
    }).generate_report()
}

fn main() {
    println!("Part 1: {}", get_diagnostic_report());
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
