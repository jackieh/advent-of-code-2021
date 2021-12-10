use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+) (\d+)$").unwrap();
}

enum Direction {
    Forward,
    Down,
    Up,
}

struct Command {
    direction: Direction,
    amount: i32,
}

#[derive(Default)]
struct SubmarinePosition {
    horizontal: i32,
    vertical: i32,
}
trait SubmarineState {
    fn get_position(self) -> SubmarinePosition;
    fn execute(self, c: &Command) -> Self;
}

#[derive(Default)]
struct Part1SubmarineState(SubmarinePosition);

impl SubmarineState for Part1SubmarineState {
    fn get_position(self) -> SubmarinePosition {
        self.0
    }

    fn execute(self, c: &Command) -> Self {
        match c.direction {
            Direction::Forward => Self(SubmarinePosition {
                horizontal: self.0.horizontal + c.amount,
                vertical: self.0.vertical,
            }),
            Direction::Down => Self(SubmarinePosition {
                horizontal: self.0.horizontal,
                vertical: self.0.vertical + c.amount,
            }),
            Direction::Up => Self(SubmarinePosition {
                horizontal: self.0.horizontal,
                vertical: self.0.vertical - c.amount,
            }),
        }
    }
}

#[derive(Default)]
struct Part2SubmarineState {
    position: SubmarinePosition,
    aim: i32,
}

impl SubmarineState for Part2SubmarineState {
    fn get_position(self) -> SubmarinePosition {
        self.position
    }

    fn execute(self, c: &Command) -> Self {
        match c.direction {
            Direction::Forward => Self {
                position: SubmarinePosition {
                    horizontal: self.position.horizontal + c.amount,
                    vertical: self.position.vertical + (self.aim * c.amount),
                },
                aim: self.aim,
            },
            Direction::Down => Self {
                position: self.position,
                aim: self.aim + c.amount,
            },
            Direction::Up => Self {
                position: self.position,
                aim: self.aim - c.amount,
            },
        }
    }
}

fn parse_direction(input_direction: &str) -> Option<Direction> {
    match input_direction {
        "forward" => Some(Direction::Forward),
        "down" => Some(Direction::Down),
        "up" => Some(Direction::Up),
        _ => None,
    }
}

fn parse_input_line(input_line: &str) -> Command {
    let cap = RE.captures(input_line).unwrap();
    Command {
        direction: parse_direction(&cap[1]).unwrap(),
        amount: cap[2].parse::<i32>().unwrap(),
    }
}

fn read_input(file_path: &str) -> Vec<Command> {
    let file = File::open(file_path).expect("Failed to read file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| parse_input_line(&line.unwrap()))
        .collect()
}

/// Calculate the product of the horizontal position and depth resulting from
/// following the commands.
fn get_solution(initial_state: impl SubmarineState, commands: &[Command]) -> i32 {
    let final_position = commands
        .iter()
        .fold(initial_state, |s, c| s.execute(c))
        .get_position();
    final_position.horizontal * final_position.vertical
}

fn part1(commands: &[Command]) -> i32 {
    get_solution(Part1SubmarineState::default(), commands)
}

fn part2(commands: &[Command]) -> i32 {
    get_solution(Part2SubmarineState::default(), commands)
}

fn main() {
    let commands = read_input("day2/src/input.txt");
    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands));
}
