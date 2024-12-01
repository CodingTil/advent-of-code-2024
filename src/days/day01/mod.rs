use std::{cmp::Ordering, fs};

pub fn main() {
	let content = read_input();
	let (vec1, vec2) = convert_input(content);
	let distance = distance(&vec1, &vec2);
	let similarity = similarity(&vec1, &vec2);

	println!("The distance between the two vectors is: {}", distance);
	println!("The similarity between the two vectors is: {}", similarity);
}

/// Read the input file to bytes
fn read_input() -> Vec<u8> {
	// format: 5 digits, 3 spaces, 5 digits
	fs::read("./src/days/day01/input").expect("Unable to read file.")
}

/// Parse the input columns to two sorted vectors
fn convert_input(content: Vec<u8>) -> (Vec<usize>, Vec<usize>) {
	let mut vec1 = Vec::new();
	let mut vec2 = Vec::new();

	let mut index = 0;
	while index < content.len() {
		// if newline, skip
		if content[index] == b'\n' {
			index += 1;
			continue;
		}

		// first 5 bytes are first number
		let num1 = fast_conversion(&content[index..(index + 5)]);
		insert_sorted(&mut vec1, num1);

		index += 5 + 3;

		// next 5 bytes are second number
		let num2 = fast_conversion(&content[index..(index + 5)]);
		insert_sorted(&mut vec2, num2);

		index += 5;
	}

	(vec1, vec2)
}

/// This function inserts a value into a sorted vector
fn insert_sorted(vec: &mut Vec<usize>, value: usize) {
	let pos = vec.binary_search(&value).unwrap_or_else(|x| x);
	vec.insert(pos, value);
}

/// This function is a fast conversion from a sequence of digit bytes (utf8) to an integer
fn fast_conversion(bytes: &[u8]) -> usize {
	let mut result = 0;
	for byte in bytes {
		result = result * 10 + (byte - b'0') as usize;
	}
	result
}

/// Determine the distance between two vectors
fn distance(vec1: &Vec<usize>, vec2: &Vec<usize>) -> usize {
	let mut distance = 0;

	for i in 0..vec1.len() {
		distance += match vec1[i].cmp(&vec2[i]) {
			Ordering::Less => vec2[i] - vec1[i],
			Ordering::Greater => vec1[i] - vec2[i],
			Ordering::Equal => 0,
		}
	}

	distance
}

/// Determine the similarity between two vectors
fn similarity(vec1: &Vec<usize>, vec2: &Vec<usize>) -> usize {
	let mut similarity = 0;

	let mut left_index = 0;
	let mut right_index = 0;

	while left_index < vec1.len() && right_index < vec2.len() {
		if vec1[left_index] == 0 {
			left_index += 1;
			continue;
		}

		match vec1[left_index].cmp(&vec2[right_index]) {
			Ordering::Less => left_index += 1,
			Ordering::Greater => right_index += 1,
			Ordering::Equal => {
				right_index += 1;
				let mut count = 1;
				while right_index < vec2.len() && vec1[left_index] == vec2[right_index] {
					count += 1;
					right_index += 1;
				}
				similarity += vec1[left_index] * count;
				left_index += 1;
			}
		}
	}

	similarity
}
