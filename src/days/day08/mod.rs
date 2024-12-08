use std::{
	collections::{HashMap, HashSet},
	fs,
};

pub fn main() {
	let (width, height, antennae_locations) = read_input();

	let antinodes = determine_all_antinodes(width, height, &antennae_locations, false);
	println!("A total of {} antinodes were found.", antinodes.len());

	let antinodes = determine_all_antinodes(width, height, &antennae_locations, true);
	println!(
		"A total of {} antinodes were found, including resonant harmonics.",
		antinodes.len()
	);
}

/// Read and parse the input file
fn read_input() -> (usize, usize, HashMap<char, HashSet<(usize, usize)>>) {
	let content = fs::read_to_string("./src/days/day08/input").expect("Unable to read file.");

	let mut width = 0;
	let mut height = 0;
	let mut antennae_locations = HashMap::new(); // map locations by frequency

	for (y, line) in content.lines().enumerate() {
		if line.is_empty() {
			break;
		}

		height = y + 1;
		width = line.len();
		for (x, c) in line.chars().enumerate() {
			if c == '.' {
				continue;
			}
			let entry = antennae_locations.entry(c).or_insert(HashSet::new());
			entry.insert((x, y));
		}
	}

	(width, height, antennae_locations)
}

/// Get the antinodes for all antennae of a given frequency
fn determine_antinodes(
	width: usize,
	height: usize,
	locatons: &HashSet<(usize, usize)>,
	include_resonant_harmonics: bool,
) -> HashSet<(usize, usize)> {
	let mut antinodes = HashSet::new();

	// loop over all pairs
	for (x1, y1) in locatons.iter() {
		let (x1_isize, y1_isize) = (*x1 as isize, *y1 as isize);

		for (x2, y2) in locatons.iter() {
			if x1 == x2 && y1 == y2 {
				continue;
			}

			let (x2_isize, y2_isize) = (*x2 as isize, *y2 as isize);

			let (dx, dy) = (x2_isize - x1_isize, y2_isize - y1_isize);

			antinodes.extend(determine_antinodes_starting_in(
				width,
				height,
				(*x1, *y1),
				(-dx, -dy),
				include_resonant_harmonics,
			));

			antinodes.extend(determine_antinodes_starting_in(
				width,
				height,
				(*x2, *y2),
				(dx, dy),
				include_resonant_harmonics,
			));
		}
	}

	antinodes
}

/// Determine all antinodes in a direction starting in a given location
fn determine_antinodes_starting_in(
	width: usize,
	height: usize,
	starting_location: (usize, usize),
	increment: (isize, isize),
	include_resonant_harmonics: bool,
) -> HashSet<(usize, usize)> {
	let mut antinodes = HashSet::new();

	if include_resonant_harmonics {
		antinodes.insert(starting_location);
	}

	let (mut x_cand, mut y_cand) = (starting_location.0 as isize, starting_location.1 as isize);

	loop {
		x_cand += increment.0;
		y_cand += increment.1;

		if x_cand < 0 || x_cand >= width as isize || y_cand < 0 || y_cand >= height as isize {
			break;
		}

		antinodes.insert((x_cand as usize, y_cand as usize));

		if !include_resonant_harmonics {
			break;
		}
	}

	antinodes
}

/// Determine all antinodes for all antennae of all frequencies
fn determine_all_antinodes(
	width: usize,
	height: usize,
	antennae_locations: &HashMap<char, HashSet<(usize, usize)>>,
	include_resonant_harmonics: bool,
) -> HashSet<(usize, usize)> {
	let mut all_antinodes = HashSet::new();

	for locations in antennae_locations.values() {
		let antinodes = determine_antinodes(width, height, locations, include_resonant_harmonics);
		all_antinodes.extend(antinodes);
	}

	all_antinodes
}
