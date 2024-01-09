use std::str::FromStr;

fn get_position(part: usize) -> u32 {
    aoc::lines::<Direction>("./data/day2.txt")
        .iter()
        .fold(Position::new(), |pos, inst| match inst {
            Direction::Forward(dist) => Position {
                distance: pos.distance + dist,
                depth: pos.depth + pos.aim * dist,
                ..pos
            },
            Direction::Up(dist) => Position {
                depth: pos.depth - dist * (part == 1) as u32, // only for p1
                aim: pos.aim - dist * (part == 2) as u32, // only for p2
                ..pos
            },
            Direction::Down(dist) => Position {
                depth: pos.depth + dist * (part == 1) as u32, // only for p1
                aim: pos.aim + dist * (part == 2) as u32, // only for p2
                ..pos
            }
        })
        .answer()
}

fn main() {
    println!("Part 1: {}", get_position(1));
    println!("Part 2: {}", get_position(2));
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