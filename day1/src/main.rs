use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
}

/// Return number of increases in each number from the previous number.
fn part1(numbers: &Vec<i32>) -> usize {
    let mut increase_count: usize = 0;
    let mut maybe_previous: Option<i32> = None;
    for current in numbers.as_slice() {
        match maybe_previous {
            Some(previous) if previous < *current => increase_count += 1,
            _ => {}
        }
        maybe_previous = Some(*current);
    }
    return increase_count;
}

/// Return number of increases in each three-measurement sliding window sum from
/// the previous three-measurement sliding window sum.
fn part2(numbers: &Vec<i32>) -> usize {
    let mut increase_count: usize = 0;
    let mut maybe_previous: Option<i32> = None;
    for window in numbers.as_slice().windows(3) {
        let current: i32 = window.iter().sum();
        match maybe_previous {
            Some(previous) if previous < current => increase_count += 1,
            _ => {}
        }
        maybe_previous = Some(current);
    }
    return increase_count;
}

fn main() {
    let numbers = read_input("day1/src/input.txt");
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers))
}
