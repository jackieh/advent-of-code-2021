use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_path: &str) -> [usize; 9] {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    let nums_list = reader
        .lines()
        .flat_map(|line| {
            let line = line.unwrap();
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<u32>>();
    let mut nums_array = [0; 9];
    for n in nums_list {
        nums_array[n as usize] += 1;
    }
    nums_array
}

fn simulate_population(pop_counts: &[usize; 9], num_days: usize) -> usize {
    let mut pop_counts = *pop_counts;
    for _ in 0..num_days {
        pop_counts.rotate_left(1);
        pop_counts[6] += pop_counts[8];
    }
    pop_counts.iter().sum()
}

fn part1(pop_counts: &[usize; 9]) -> usize {
    simulate_population(pop_counts, 80)
}

fn part2(pop_counts: &[usize; 9]) -> usize {
    simulate_population(pop_counts, 256)
}

fn main() {
    let pop_counts = read_input("day6/src/input.txt");
    println!("Part 1: {}", part1(&pop_counts));
    println!("Part 2: {}", part2(&pop_counts));
}
