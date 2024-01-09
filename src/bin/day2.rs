use std::str::FromStr;

fn part_one() -> u32 {
    aoc::lines::<Direction>("./data/day2.txt")
        .iter()
        .fold(Position::new(), |pos, inst| match inst {
            Direction::Forward(dist) => Position {
                distance: pos.distance + dist,
                ..pos
            },
            Direction::Up(dist) => Position {
                depth: pos.depth - dist,
                ..pos
            },
            Direction::Down(dist) => Position {
                depth: pos.depth + dist,
                ..pos
            }
        })
        .answer()
}

fn part_two() -> u32 {
    aoc::lines::<Direction>("./data/day2.txt")
        .iter()
        .fold(Position::new(), |pos, inst| match inst {
            Direction::Forward(dist) => Position {
                distance: pos.distance + dist,
                depth: pos.depth + pos.aim * dist,
                ..pos
            },
            Direction::Up(dist) => Position {
                aim: pos.aim - dist,
                ..pos
            },
            Direction::Down(dist) => Position {
                aim: pos.aim + dist,
                ..pos
            }
        })
        .answer()
}

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split_once(" ").unwrap();

        match values.0 {
            "forward" => Ok(Direction::Forward(values.1.parse::<u32>().unwrap())),
            "up" => Ok(Direction::Up(values.1.parse::<u32>().unwrap())),
            "down" => Ok(Direction::Down(values.1.parse::<u32>().unwrap())),
            _ => Err(()),
        }
    }
}

struct Position {
    distance: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn new() -> Self {
        Position { distance: 0, depth: 0, aim: 0 }
    }

    fn answer(&self) -> u32 {
        self.distance * self.depth
    }
}