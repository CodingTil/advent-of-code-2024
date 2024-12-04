use regex::Regex;
use std::fs;

pub fn main() {
	let content = read_input();
	let grid = create_grid(&content);
	let xmas_occurrences = count_xmas_occurrences(&grid);
	let x_mas_occurrences = count_x_mas_occurrences(&grid);

	println!("XMAS occurrences: {}", xmas_occurrences);
	println!("X-MAS occurrences: {}", x_mas_occurrences);
}

/// Read the input file to bytes
fn read_input() -> String {
	fs::read_to_string("./src/days/day04/input").expect("Unable to read file.")
}

/// Create a grid from the input
fn create_grid(content: &String) -> Vec<Vec<char>> {
	let mut grid = Vec::new();
	for line in content.lines() {
		let mut row = Vec::new();
		for c in line.chars() {
			row.push(c);
		}
		grid.push(row);
	}
	grid
}

/// Count the number of occurrences of the word "XMAS" in the grid
fn count_xmas_occurrences(grid: &[Vec<char>]) -> usize {
	let rows = grid.len();
	let cols = grid[0].len();
	let target = "XMAS".chars().collect::<Vec<char>>();
	let target_len = target.len();

	let mut count = 0;

	for row in 0..rows {
		for col in 0..cols {
			// Check all 8 possible directions
			let directions = [
				(0, 1),   // right
				(0, -1),  // left
				(1, 0),   // down
				(-1, 0),  // up
				(1, 1),   // diagonal down-right
				(-1, -1), // diagonal up-left
				(1, -1),  // diagonal down-left
				(-1, 1),  // diagonal up-right
			];

			for &(dr, dc) in &directions {
				if matches_target(grid, row, col, dr, dc, &target, target_len) {
					count += 1;
				}
			}
		}
	}

	count
}

/// Check if the target word matches the grid starting at the given position and moving in the given direction
fn matches_target(
	grid: &[Vec<char>],
	start_row: usize,
	start_col: usize,
	increment_row: isize,
	increment_column: isize,
	target: &[char],
	target_len: usize,
) -> bool {
	let rows = grid.len();
	let cols = grid[0].len();

	for i in 0..target_len {
		let r = start_row as isize + i as isize * increment_row;
		let c = start_col as isize + i as isize * increment_column;

		if r < 0 || r >= rows as isize || c < 0 || c >= cols as isize {
			return false;
		}

		if grid[r as usize][c as usize] != target[i] {
			return false;
		}
	}

	true
}

/// Count the number of occurrences of the pattern X-"MAS" in the grid
fn count_x_mas_occurrences(grid: &[Vec<char>]) -> usize {
	let rows = grid.len();
	let cols = grid[0].len();

	let mut count = 0;

	// Only iterate over inner characters
	for row in 1..rows - 1 {
		for col in 1..cols - 1 {
			if grid[row][col] == 'A' && is_valid_x_mas(grid, row, col) {
				count += 1;
			}
		}
	}

	count
}

/// Check if the pattern X-"MAS" is valid at the given position
fn is_valid_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
	// check that top left to bottom right diagonal has one m and one s
	if !((grid[row - 1][col - 1] == 'M' && grid[row + 1][col + 1] == 'S')
		|| (grid[row - 1][col - 1] == 'S' && grid[row + 1][col + 1] == 'M'))
	{
		return false;
	}

	if !((grid[row - 1][col + 1] == 'M' && grid[row + 1][col - 1] == 'S')
		|| (grid[row - 1][col + 1] == 'S' && grid[row + 1][col - 1] == 'M'))
	{
		return false;
	}

	return true;
}
