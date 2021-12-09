use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

/// Return number of increases in each number from the previous number.
fn part1(numbers: &[i32]) -> usize {
    numbers
        .iter()
        .zip(numbers.iter().skip(1))
        .filter(|(a, b)| a < b)
        .count()
}

/// Return number of increases in each three-measurement sliding window sum from
/// the previous three-measurement sliding window sum.
fn part2(numbers: &[i32]) -> usize {
    let window_sums: Vec<i32> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    part1(&window_sums)
}

fn main() {
    let numbers = read_input("day1/src/input.txt");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}
