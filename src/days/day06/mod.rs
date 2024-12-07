use std::{
	collections::{HashMap, HashSet},
	fs,
};

/// Direction enum with (x_increment, y_increment)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	fn get_increment(&self) -> (isize, isize) {
		match *self {
			Direction::Up => (0, -1),
			Direction::Down => (0, 1),
			Direction::Left => (-1, 0),
			Direction::Right => (1, 0),
		}
	}

	fn turn(&self) -> Self {
		match *self {
			Direction::Up => Direction::Right,
			Direction::Right => Direction::Down,
			Direction::Down => Direction::Left,
			Direction::Left => Direction::Up,
		}
	}
}

pub fn main() {
	let (current_position, current_direction, obstacles, grid_size) = read_input();

	let unique_positions =
		count_unique_positions(current_position, &current_direction, &obstacles, grid_size);
	println!("Unique positions visited: {}", unique_positions);

	let loop_positions =
		count_loop_positions(current_position, &current_direction, &obstacles, grid_size);
	println!(
		"Positions for one additional obstruction to cause a loop: {}",
		loop_positions
	);
}

/// Read the input file into a tuple of (current_position, current_direction, obstacles, grid_size)
fn read_input() -> (
	(usize, usize),
	Direction,
	HashSet<(usize, usize)>,
	(usize, usize),
) {
	let content = fs::read_to_string("./src/days/day06/input").expect("Unable to read file.");

	let mut current_position = (0, 0);
	let current_direction = Direction::Up;
	let mut obstacles = HashSet::new();
	let mut grid_width = 0;

	let mut y_pos = 0;
	for line in content.lines() {
		let mut x_pos = 0;
		for c in line.chars() {
			if c == '#' {
				obstacles.insert((x_pos, y_pos));
			} else if c == '^' {
				current_position = (x_pos, y_pos);
			}
			x_pos += 1;
		}
		grid_width = x_pos;
		y_pos += 1;
	}

	(
		current_position,
		current_direction,
		obstacles,
		(grid_width, y_pos),
	)
}

/// Count the number of unique positions the guard will visit before leaving the map
fn count_unique_positions(
	current_position: (usize, usize),
	current_direction: &Direction,
	obstacles: &HashSet<(usize, usize)>,
	grid_size: (usize, usize),
) -> usize {
	let mut visited_positions = HashSet::new();
	let mut current_position = current_position;
	let mut current_direction = current_direction.clone();

	loop {
		visited_positions.insert(current_position);

		let (x, y) = current_position;
		let (dx, dy) = current_direction.get_increment();
		let new_position = (x as isize + dx, y as isize + dy);

		if new_position.0 < 0
			|| new_position.0 >= grid_size.0 as isize
			|| new_position.1 < 0
			|| new_position.1 >= grid_size.1 as isize
		{
			break;
		}

		let new_position = (new_position.0 as usize, new_position.1 as usize);

		if obstacles.contains(&new_position) {
			current_direction = current_direction.turn();
		} else {
			current_position = new_position;
		}
	}

	visited_positions.len()
}

/// Count the number of positions for one additional obstruction to cause a loop
fn count_loop_positions(
	current_position: (usize, usize),
	current_direction: &Direction,
	obstacles: &HashSet<(usize, usize)>,
	grid_size: (usize, usize),
) -> usize {
	let mut visited_positions: HashSet<((usize, usize), Direction)> = HashSet::new();
	let mut count = 0;
	let initial_position = current_position;
	let initial_direction = current_direction.clone();
	let mut current_position = current_position;
	let mut current_direction = current_direction.clone();
	let mut all_obstacles = obstacles.clone();
	let mut positions_of_additional_obstacles_tries: HashSet<(usize, usize)> = HashSet::new();

	loop {
		visited_positions.insert((current_position, current_direction.clone()));

		let (x, y) = current_position;
		let (dx, dy) = current_direction.get_increment();
		let new_position = (x as isize + dx, y as isize + dy);

		if new_position.0 < 0
			|| new_position.0 >= grid_size.0 as isize
			|| new_position.1 < 0
			|| new_position.1 >= grid_size.1 as isize
		{
			break;
		}

		let new_position = (new_position.0 as usize, new_position.1 as usize);

		if initial_position != new_position {
			// check if the current position is already tried as an additional obstacle
			if positions_of_additional_obstacles_tries.insert(new_position) {
				// temporaily check if adding an obstacle at the current position would cause a loop
				if all_obstacles.insert(new_position) {
					if is_loop(
						initial_position,
						&initial_direction,
						&all_obstacles,
						grid_size,
					) {
						count += 1;
					}
					assert!(all_obstacles.remove(&new_position));
				}
			}
		}

		if obstacles.contains(&new_position) {
			current_direction = current_direction.turn();
		} else {
			current_position = new_position;
		}
	}

	count
}

/// Function to detect a loop
fn is_loop(
	current_position: (usize, usize),
	current_direction: &Direction,
	all_obstacles: &HashSet<(usize, usize)>,
	grid_size: (usize, usize),
) -> bool {
	let mut visited_positions: HashSet<((usize, usize), Direction)> = HashSet::new();
	let mut current_position = current_position;
	let mut current_direction = current_direction.clone();

	loop {
		if !visited_positions.insert((current_position, current_direction.clone())) {
			return true;
		}

		let (x, y) = current_position;
		let (dx, dy) = current_direction.get_increment();
		let new_position = (x as isize + dx, y as isize + dy);

		if new_position.0 < 0
			|| new_position.0 >= grid_size.0 as isize
			|| new_position.1 < 0
			|| new_position.1 >= grid_size.1 as isize
		{
			break;
		}

		let new_position = (new_position.0 as usize, new_position.1 as usize);

		if all_obstacles.contains(&new_position) {
			current_direction = current_direction.turn();
		} else {
			current_position = new_position;
		}
	}

	false
}
