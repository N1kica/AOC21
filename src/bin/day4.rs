fn main() {
    let part_one = aoc::split::<i32>("./data/day4-part1.txt", ",");  
    let part_two = aoc::boards::<i32>("./data/day4-part2.txt", 5);

    println!("Parsed 1:");
    println!("{:?}", part_one);
    println!("Parsed 2:");
    println!("{:?}", part_two);
}