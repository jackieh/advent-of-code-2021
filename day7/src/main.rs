use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_path: &str) -> Vec<u32> {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>()
}

fn abs_diff(a: u32, b: u32) -> u32 {
    (a as i32 - b as i32).unsigned_abs()
}

fn part1(nums: &[u32]) -> u32 {
    let mut nums = nums.to_owned();
    nums.sort();
    let median = nums[nums.len() / 2];
    nums.iter().map(|n| abs_diff(*n, median)).sum()
}

fn part2(nums: &[u32]) -> u32 {
    let sum: u32 = nums.iter().sum();
    let avg = sum as f32 / nums.len() as f32;
    fn get_fuel_cost(num_steps: u32) -> u32 {
        (num_steps * (num_steps + 1)) / 2
    }
    let get_total_fuel_cost = |position: u32| {
        nums.iter()
            .map(|n| get_fuel_cost(abs_diff(*n, position)))
            .sum::<u32>()
    };
    let floor = avg.floor() as u32;
    let ceil = avg.ceil() as u32;
    get_total_fuel_cost(floor).min(get_total_fuel_cost(ceil))
}

fn main() {
    let nums_list = read_input("day7/src/input.txt");
    println!("Part 1: {}", part1(&nums_list));
    println!("Part 2: {}", part2(&nums_list));
}
