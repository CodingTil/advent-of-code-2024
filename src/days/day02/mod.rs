use std::fs;

pub fn main() {
	let content = read_input();
	let safe_count = count_safe_reports(&content);
	let dampened_safe_count = count_dampened_safe_reports(&content);

	println!("Safe reports: {}", safe_count);
	println!("Dampened safe reports: {}", dampened_safe_count);
}

/// Read the input file to bytes
fn read_input() -> String {
	fs::read_to_string("./src/days/day02/input").expect("Unable to read file.")
}

fn count_safe_reports(content: &String) -> usize {
	let mut count = 0;

	for line in content.lines() {
		let report = line
			.split_whitespace()
			.map(|x| x.parse::<usize>().unwrap())
			.collect::<Vec<usize>>();

		if is_report_safe(&report) {
			count += 1;
		}
	}

	count
}

fn count_dampened_safe_reports(content: &String) -> usize {
	let mut count = 0;

	for line in content.lines() {
		let mut report = line
			.split_whitespace()
			.map(|x| x.parse::<usize>().unwrap())
			.collect::<Vec<usize>>();

		if is_report_safe_dampened(&mut report) {
			count += 1;
		}
	}

	count
}

/// a report only counts as safe if both of the following are true:
/// - the levels are either all increasing or all decreasing
/// - any two adjacent levels differ by at least one and at most three
fn is_report_safe(report: &Vec<usize>) -> bool {
	if report.len() <= 1 {
		return true;
	}

	let first = report[0];
	let second = report[1];

	if first < second {
		for i in 0..report.len() - 1 {
			if report[i] > report[i + 1] {
				return false;
			}
			let diff = report[i + 1] - report[i];
			if diff < 1 || diff > 3 {
				return false;
			}
		}
	} else {
		for i in 0..report.len() - 1 {
			if report[i] < report[i + 1] {
				return false;
			}
			let diff = report[i] - report[i + 1];
			if diff < 1 || diff > 3 {
				return false;
			}
		}
	}

	true
}

/// a report only counts as dampened safe if removing at most one level from the report makes it safe
fn is_report_safe_dampened(report: &mut Vec<usize>) -> bool {
	if report.len() <= 1 {
		return true;
	}

	if is_report_safe(&report) {
		return true;
	}

	for i in 0..report.len() {
		let removed = report.remove(i);

		if is_report_safe(report) {
			return true;
		}

		report.insert(i, removed);
	}

	false
}
