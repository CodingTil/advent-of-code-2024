use std::{
	collections::{HashMap, HashSet},
	fs,
};

pub fn main() {
	let (rules, updates) = read_input();
	let dependency_map = rules_to_dependency_map(&rules);

	let sum_ordered = sum_middle_ordered_updates(&updates, &dependency_map);
	let sum_unordered = sum_middle_unordered_updates(&updates, &dependency_map);

	println!("Sum of middle ordered updates: {}", sum_ordered);
	println!("Sum of middle unordered updates: {}", sum_unordered);
}

/// Read the input file into page ordering rules and updates
fn read_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
	let content = fs::read_to_string("./src/days/day05/input").expect("Unable to read file.");

	// the empty line separates the rules from the updates
	let parts: Vec<&str> = content.split("\n\n").collect();

	// rules format: number|number
	let rules = parts[0]
		.lines()
		.map(|line| {
			let mut parts = line.split("|");
			let start = parts.next().unwrap().parse().unwrap();
			let end = parts.next().unwrap().parse().unwrap();
			(start, end)
		})
		.collect();

	// updates format: list of comma separated numbers
	let updates = parts[1]
		.lines()
		.map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
		.collect();

	(rules, updates)
}

/// Convert the rules into dependency map
fn rules_to_dependency_map(rules: &[(usize, usize)]) -> HashMap<usize, HashSet<usize>> {
	let mut dependency_map = HashMap::new();

	for (before, after) in rules {
		dependency_map
			.entry(*before)
			.or_insert(HashSet::new())
			.insert(*after);
	}

	dependency_map
}

/// Check if the update is ordered according to the dependency map
fn is_update_ordered(update: &Vec<usize>, dependency_map: &HashMap<usize, HashSet<usize>>) -> bool {
	for i in 0..update.len() - 1 {
		let before = update.get(i).unwrap();
		for j in i + 1..update.len() {
			let after = update.get(j).unwrap();
			// after -> before should not be in the dependency map
			if dependency_map.contains_key(after)
				&& dependency_map.get(after).unwrap().contains(before)
			{
				return false;
			}
		}
	}
	true
}

/// Sum the middle number of all ordered updates
fn sum_middle_ordered_updates(
	updates: &Vec<Vec<usize>>,
	dependency_map: &HashMap<usize, HashSet<usize>>,
) -> usize {
	let mut sum = 0;
	for update in updates {
		if is_update_ordered(&update, dependency_map) {
			let middle_number = update.get(update.len() / 2).unwrap();
			sum += middle_number;
		}
	}
	sum
}

/// Topological sort an update
fn topological_sort(
	update: &Vec<usize>,
	dependency_map: &HashMap<usize, HashSet<usize>>,
) -> Vec<usize> {
	let mut sorted = Vec::new();
	let mut to_visit = update.clone();

	while !to_visit.is_empty() {
		let next = to_visit[to_visit.len() - 1];
		visit(next, dependency_map, &mut to_visit, &mut sorted);
	}

	sorted
}

/// Visit a node in the topological sort
fn visit(
	next: usize,
	outgoing_edges: &HashMap<usize, HashSet<usize>>,
	to_visit: &mut Vec<usize>,
	sorted: &mut Vec<usize>,
) {
	if !to_visit.contains(&next) {
		return;
	}

	let index = to_visit.iter().position(|x| *x == next).unwrap();
	to_visit.remove(index);

	if let Some(dependencies) = outgoing_edges.get(&next) {
		for dependency in dependencies {
			visit(*dependency, outgoing_edges, to_visit, sorted);
		}
	}

	sorted.push(next);
}

/// Sum the middle number of all unordered updates
fn sum_middle_unordered_updates(
	updates: &Vec<Vec<usize>>,
	dependency_map: &HashMap<usize, HashSet<usize>>,
) -> usize {
	let mut sum = 0;
	for update in updates {
		if !is_update_ordered(&update, dependency_map) {
			let sorted = topological_sort(&update, dependency_map);
			let middle_number = sorted.get(sorted.len() / 2).unwrap();
			sum += middle_number;
		}
	}
	sum
}
