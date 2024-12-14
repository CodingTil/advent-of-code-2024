use std::fs;

pub fn main() {
	let disk_map = read_input();
	let blocks = to_blocks(&disk_map);
	let compacted_blocks = compact_blocks(&blocks);
	let checksum = compacted_blocks_checksum(&compacted_blocks);

	println!("Checksum: {}", checksum);

	let blocks_v2 = to_blocks_v2(&disk_map);
	let defragmented_files = defragment_files(&blocks_v2);
	let checksum_v2 = compacted_files_checksum(&defragmented_files);

	println!("Checksum v2: {}", checksum_v2);
}

/// Read and parse the input file
fn read_input() -> Vec<usize> {
	let content = fs::read_to_string("./src/days/day09/input").expect("Unable to read file.");

	let mut disk_map: Vec<usize> = Vec::new();

	for (y, line) in content.lines().enumerate() {
		if line.is_empty() {
			break;
		}

		// every char is a usize to be added
		for c in line.chars() {
			disk_map.push(c.to_string().parse::<usize>().unwrap());
		}
	}

	disk_map
}

/// Convert to blocks
fn to_blocks(disk_map: &Vec<usize>) -> Vec<Option<usize>> {
	let mut blocks: Vec<Option<usize>> = Vec::new();

	for (id_2, length) in disk_map.iter().enumerate() {
		if id_2 % 2 == 0 {
			let id = id_2 / 2;
			for _ in 0..*length {
				blocks.push(Some(id));
			}
		} else {
			for _ in 0..*length {
				blocks.push(None);
			}
		}
	}

	blocks
}

/// Compact Blocks
fn compact_blocks(blocks: &Vec<Option<usize>>) -> Vec<usize> {
	let mut compacted_blocks: Vec<usize> = Vec::new();
	let mut blocks = blocks.clone();

	while !blocks.is_empty() {
		match blocks.remove(0) {
			Some(id) => {
				compacted_blocks.push(id);
			}
			None => loop {
				match blocks.pop() {
					Some(Some(id)) => {
						compacted_blocks.push(id);
						break;
					}
					Some(None) => {
						continue;
					}
					None => {
						break;
					}
				}
			},
		}
	}

	compacted_blocks
}

/// compacted blocks checksum
fn compacted_blocks_checksum(compacted_blocks: &Vec<usize>) -> usize {
	let mut checksum = 0;

	for (position, id) in compacted_blocks.iter().enumerate() {
		checksum += position * id;
	}

	checksum
}

/// id, start, length
#[derive(Clone)]
struct File(usize, usize, usize);

/// start, length
#[derive(Clone)]
struct FreeBlock(usize, usize);

enum Block {
	File(File),
	FreeBlock(FreeBlock),
}

/// convert to blocks: Block
fn to_blocks_v2(disk_map: &Vec<usize>) -> Vec<Block> {
	let mut blocks: Vec<Block> = Vec::new();
	let mut start = 0;

	for (id_2, length_2) in disk_map.iter().enumerate() {
		if id_2 % 2 == 0 {
			let id = id_2 / 2;
			let length = *length_2;
			blocks.push(Block::File(File(id, start, length)));
			start += length;
		} else {
			let length = *length_2;
			blocks.push(Block::FreeBlock(FreeBlock(start, length)));
			start += length;
		}
	}

	blocks
}

/// defragement the files (entire file moves, not just blocks)
fn defragment_files(blocks: &Vec<Block>) -> Vec<File> {
	let mut defragmented_files: Vec<File> = Vec::new();

	let mut free_blocks: Vec<FreeBlock> = Vec::new();
	for block in blocks.iter() {
		match block {
			Block::FreeBlock(free_block) => {
				free_blocks.push(free_block.clone());
			}
			_ => {}
		}
	}

	for block in blocks.iter().rev() {
		match block {
			Block::File(file) => {
				let id = file.0;
				let start = file.1;
				let length = file.2;

				let mut file_to_add = file.clone();

				for free_block in free_blocks.iter_mut() {
					if free_block.0 < start && free_block.1 >= length {
						file_to_add = File(id, free_block.0, length);

						free_block.0 += length;
						free_block.1 -= length;

						break;
					}
				}

				defragmented_files.push(file_to_add);
			}
			Block::FreeBlock(_) => {
				continue;
			}
		}
	}

	defragmented_files
}

fn compacted_files_checksum(defragmented_files: &Vec<File>) -> usize {
	let mut checksum = 0;

	for file in defragmented_files.iter() {
		let id = file.0;
		let mut start = file.1;
		let length = file.2;

		for _ in 0..length {
			checksum += start * id;
			start += 1;
		}
	}

	checksum
}
