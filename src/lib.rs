use std::str::FromStr;

pub fn read_lines<T>(path: &str) -> Vec<T>
where 
    T: FromStr
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading a file")
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}

pub fn read_chars<T, F>(path: &str, f: F) -> Vec<Vec<T>>
where 
    T: FromStr,
    F: Fn(char) -> Option<T>
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading a file")
        .lines()
        .map(|line| line
            .chars()
            .filter_map(|c| f(c))
            .collect())
        .collect()
}
