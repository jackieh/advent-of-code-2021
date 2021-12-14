use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input_line(input_line: &str) -> Vec<u32> {
    input_line.chars().map(|c| c.to_digit(2).unwrap()).collect()
}

fn read_input(file_path: &str) -> Vec<Vec<u32>> {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| parse_input_line(&line.unwrap()))
        .collect()
}

fn part1(binary_numbers: &[Vec<u32>]) -> u32 {
    let num_rows = binary_numbers.len();
    let num_cols = binary_numbers[0].len();
    let sums: Vec<u32> = (0..num_cols)
        .map(|c| (0..num_rows).fold(0, |s, r| s + binary_numbers[r][c]))
        .collect();
    let gamma_bits: Vec<u32> = sums
        .iter()
        .map(|s| (s * 2 > num_rows as u32) as u32)
        .collect();
    let gamma = gamma_bits.iter().fold(0, |d, b| d * 2 + b);
    let epsilon = gamma_bits.iter().fold(0, |d, b| d * 2 + (1 ^ b));
    gamma * epsilon
}

fn get_filtered_number(binary_numbers: &[Vec<u32>], requires_most_common_bit: bool) -> u32 {
    let num_cols = binary_numbers[0].len();
    let mut remaining_numbers: Vec<&Vec<u32>> = binary_numbers.iter().collect();
    for c in 0..num_cols {
        let num_rows = remaining_numbers.len();
        if num_rows == 1 {
            break;
        }
        let sum = (0..num_rows).fold(0, |s, r| s + remaining_numbers[r][c]);
        let most_common_bit = (sum * 2 >= num_rows as u32) as u32;
        let required_bit = most_common_bit ^ (requires_most_common_bit as u32);
        remaining_numbers = remaining_numbers
            .iter()
            .filter(|row| row[c] == required_bit)
            .copied()
            .collect();
    }
    remaining_numbers[0].iter().fold(0, |d, b| d * 2 + b)
}

fn part2(binary_numbers: &[Vec<u32>]) -> u32 {
    let oxygen_generator_rating = get_filtered_number(binary_numbers, true);
    let co2_scrubber_rating = get_filtered_number(binary_numbers, false);
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let binary_numbers = read_input("day3/src/input.txt");
    println!("Part 1: {}", part1(&binary_numbers));
    println!("Part 2: {}", part2(&binary_numbers));
}
