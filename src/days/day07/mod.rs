use std::{collections::HashSet, fs};

#[derive(Debug, Eq, Hash, PartialEq)]
enum Operator {
	Add,
	Multiply,
	Concatenate,
}

impl Operator {
	fn apply(&self, a: usize, b: usize) -> usize {
		match self {
			Operator::Add => a + b,
			Operator::Multiply => a * b,
			Operator::Concatenate => {
				// 12 || 345 => 12345
				let mut a = a;
				let mut b_copy = b;
				while b_copy > 0 {
					a *= 10;
					b_copy /= 10;
				}
				a + b
			}
		}
	}
}

pub fn main() {
	let equations = read_input();

	let sum_ways_solvable = sum_ways_to_solve(&equations, {
		let mut operators = HashSet::new();
		operators.insert(Operator::Add);
		operators.insert(Operator::Multiply);
		operators
	});
	println!("Sum of ways to solve: {}", sum_ways_solvable);

	let sum_ways_solvable_with_concatenation = sum_ways_to_solve(&equations, {
		let mut operators = HashSet::new();
		operators.insert(Operator::Add);
		operators.insert(Operator::Multiply);
		operators.insert(Operator::Concatenate);
		operators
	});
	println!(
		"Sum of ways to solve with concatenation: {}",
		sum_ways_solvable_with_concatenation
	);
}

/// Read the input file into a Set of tuples (usize, Vec<usize>)
fn read_input() -> HashSet<(usize, Vec<usize>)> {
	// each line has the format
	// <number>: <number> <number>...
	let content = fs::read_to_string("./src/days/day07/input").expect("Unable to read file.");

	let mut equations = HashSet::new();
	for line in content.lines() {
		let mut parts = line.split(": ");
		let number = parts.next().unwrap().parse::<usize>().unwrap();
		let mut numbers = Vec::new();
		for number in parts.next().unwrap().split(" ") {
			numbers.push(number.parse::<usize>().unwrap());
		}
		equations.insert((number, numbers));
	}

	equations
}

/// Count ways to solve an equation by placing in + or * operators (left to right evaluation)
fn count_ways_to_solve(equation: &(usize, Vec<usize>), operators: &HashSet<Operator>) -> usize {
	let (target, numbers) = equation;

	// If there's only one number, check if it matches the target
	if numbers.len() == 1 {
		return if numbers[0] == *target { 1 } else { 0 };
	}

	count_ways_to_solve_from(equation, &operators, numbers[0], 1)
}

fn count_ways_to_solve_from(
	equation: &(usize, Vec<usize>),
	operators: &HashSet<Operator>,
	running_result: usize,
	index: usize,
) -> usize {
	let (target, numbers) = equation;

	if numbers.len() == index {
		return if running_result == *target { 1 } else { 0 };
	}

	let mut count = 0;

	for operator in operators {
		let new_running_result = operator.apply(running_result, numbers[index]);
		if new_running_result <= *target {
			count += count_ways_to_solve_from(equation, &operators, new_running_result, index + 1);
		}
	}

	count
}

/// Add count ways for solvable equations
fn sum_ways_to_solve(
	equations: &HashSet<(usize, Vec<usize>)>,
	operators: HashSet<Operator>,
) -> usize {
	let mut sum = 0;

	for equation in equations {
		if count_ways_to_solve(equation, &operators) > 0 {
			sum += equation.0;
		}
	}

	sum
}
