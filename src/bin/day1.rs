fn descending_widnow(size: usize) -> usize {
    aoc::read_lines::<u32>("./data/day1.txt")
        .windows(size)
        .filter(|win| win[0] < win[size - 1])
        .count()
}

fn main() {
    println!("Part 1: {}", descending_widnow(2));
    println!("Part 1: {}", descending_widnow(4));
}