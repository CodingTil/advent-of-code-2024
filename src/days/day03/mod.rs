use regex::Regex;
use std::fs;

pub fn main() {
	let content = read_input();
	let sum = extract_mul_and_sum(&content);
	let sum_with_conditions = extract_and_sum_with_conditions(&content);

	println!("Sum: {}", sum);
	println!("Sum with conditions: {}", sum_with_conditions);
}

/// Read the input file to bytes
fn read_input() -> String {
	fs::read_to_string("./src/days/day03/input").expect("Unable to read file.")
}

/// Extract the values from the input string
fn extract_mul_and_sum(input: &String) -> usize {
	let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
	let mut sum = 0;

	for cap in re.captures_iter(input) {
		let x: usize = cap[1].parse().unwrap();
		let y: usize = cap[2].parse().unwrap();

		sum += x * y;
	}

	sum
}

/// Extract values and sum with conditions
fn extract_and_sum_with_conditions(input: &String) -> usize {
	// Match `do()`, `don't()`, and valid `mul(X,Y)` instructions
	let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

	let mut sum = 0;
	let mut enabled = true; // Start with `mul` instructions enabled

	for cap in re.captures_iter(input) {
		if let Some(do_instruction) = cap.get(0).filter(|m| m.as_str() == "do()") {
			enabled = true; // Enable future `mul` instructions
		} else if let Some(dont_instruction) = cap.get(0).filter(|m| m.as_str() == "don't()") {
			enabled = false; // Disable future `mul` instructions
		} else if let (Some(x_match), Some(y_match)) = (cap.get(1), cap.get(2)) {
			if enabled {
				let x: usize = x_match.as_str().parse().unwrap();
				let y: usize = y_match.as_str().parse().unwrap();
				sum += x * y; // Add result of the multiplication
			}
		}
	}

	sum
}
