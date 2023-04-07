use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Copy, Clone)]
struct BingoTile {
    number: u32,
    marked: bool,
}

impl BingoTile {
    fn new(n: u32) -> Self {
        BingoTile {
            number: n,
            marked: false,
        }
    }
}

#[derive(Clone)]
struct BingoBoard {
    tiles: [[BingoTile; 5]; 5],
    bingo: bool,
}

impl BingoBoard {
    fn new(tiles: [[BingoTile; 5]; 5]) -> Self {
        BingoBoard {
            tiles,
            bingo: false,
        }
    }

    // Mark any matching tiles with the given number.
    fn mark(&mut self, n: u32) {
        self.tiles.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|tile| {
                if tile.number == n {
                    tile.marked = true;
                }
            })
        });
    }

    // The board has a bingo if any row or column is fully marked. Diagonals
    // are not considered.
    fn check_bingo(&mut self) {
        self.bingo |= (0..5).any(|r| (0..5).all(|c| self.tiles[r][c].marked));
        self.bingo |= (0..5).any(|c| (0..5).all(|r| self.tiles[r][c].marked));
    }
}

// Parse a text file into problem input. First line will contain a list of
// numbers, the second line will be a blank line, and the rest of the lines
// will contain 5x5 bingo boards separated by blank lines.
fn read_input(file_path: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let boards: Vec<BingoBoard> = lines
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|chunk| {
            let mut tiles: [[BingoTile; 5]; 5] = [[BingoTile::default(); 5]; 5];
            for (i, line) in chunk.iter().skip(1).enumerate() {
                let row = line
                    .split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                for (j, n) in row.iter().enumerate() {
                    tiles[i][j] = BingoTile::new(*n);
                }
            }
            BingoBoard::new(tiles)
        })
        .collect();

    (numbers, boards)
}

// Return the first board that wins the soonest when the numbers are called,
// and the number that is called when the board wins.
fn get_first_winning_board(numbers: &[u32], boards: &[BingoBoard]) -> (u32, BingoBoard) {
    let mut boards: Vec<BingoBoard> = boards.to_owned();
    let mut winning_board = None;
    let mut winning_number = None;
    for n in numbers {
        boards.iter_mut().for_each(|board| board.mark(*n));
        boards.iter_mut().for_each(|board| board.check_bingo());
        if boards.iter().any(|board| board.bingo) {
            winning_number = Some(*n);
            winning_board = Some(
                boards
                    .iter()
                    .find(|board| board.bingo)
                    .unwrap()
                    .clone(),
            );
            break;
        }
    }
    (winning_number.unwrap(), winning_board.unwrap())
}

// Return the first board that wins the latest when the numbers are called,
// and the number that is called when the board wins.
fn get_last_winning_board(numbers: &[u32], boards: &[BingoBoard]) -> (u32, BingoBoard) {
    let mut boards: Vec<BingoBoard> = boards.to_owned();
    let mut winning_number = None;
    let mut winning_board = None;
    for n in numbers {
        boards.iter_mut().for_each(|board| board.mark(*n));
        boards.iter_mut().for_each(|board| board.check_bingo());
        if boards.iter().any(|board| board.bingo) {
            winning_number = Some(*n);
            winning_board = Some(
                boards
                    .iter()
                    .find(|board| board.bingo)
                    .unwrap()
                    .clone(),
            );
            boards.retain(|board| !board.bingo);
        }
    }
    (winning_number.unwrap(), winning_board.unwrap())
}

fn get_sum_of_unmarked_numbers(board: &BingoBoard) -> u32 {
    board
        .tiles
        .iter()
        .flatten()
        .filter(|tile| !tile.marked)
        .map(|tile| tile.number)
        .sum()
}

// Part 1: Return the first number to get a bingo times the sum of all unmarked numbers of the first winning board.
fn part1(numbers: &[u32], boards: &[BingoBoard]) -> u32 {
    let (winning_number, winning_board) = get_first_winning_board(numbers, boards);
    winning_number * get_sum_of_unmarked_numbers(&winning_board)
}

// Part 2: Return the last number to get a bingo times the sum of all unmarked numbers of the last winning board.
fn part2(numbers: &[u32], boards: &[BingoBoard]) -> u32 {
    let (winning_number, winning_board) = get_last_winning_board(numbers, boards);
    winning_number * get_sum_of_unmarked_numbers(&winning_board)
}

fn main() {
    let (numbers, boards): (Vec<u32>, Vec<BingoBoard>) = read_input("day4/src/input.txt");
    println!("Part 1: {}", part1(&numbers, &boards));
    println!("Part 2: {}", part2(&numbers, &boards));
}
