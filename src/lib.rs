use std::str::FromStr;
use itertools::Itertools;

pub fn lines<T>(path: &str) -> Vec<T>
where 
    T: FromStr
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect()
}

pub fn chars<T, F>(path: &str, f: F) -> Vec<T>
where 
    F: Fn(char) -> Option<T>
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .chars()
        .filter_map(|c| f(c))
        .collect()
}

pub fn chars_per_line<T, F>(path: &str, f: F) -> Vec<Vec<T>>
where 
    T: FromStr,
    F: Fn(char) -> Option<T>
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line
            .chars()
            .filter_map(|c| f(c))
            .collect())
        .collect()
}

pub fn split<T>(path: &str, pat: &str) -> Vec<T>
where
    T: FromStr
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .split(pat)
        .into_iter()
        .filter_map(|c| c.parse::<T>().ok())
        .collect()
}

pub fn boards<T>(path: &str, size: usize) -> Vec<Vec<Vec<T>>>
where
    T: FromStr
{
    std::fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .lines()
        .filter(|&line| !line.is_empty())
        .chunks(size)
        .into_iter()
        .map(|chunk| chunk.map(|line| line
            .split_whitespace()
            .into_iter()
            .filter_map(|x| x.parse::<T>().ok()).collect_vec())
            .collect_vec()
        )
        .collect_vec()
}