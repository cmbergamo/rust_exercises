use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
	let path = Path::new("E:\\trabalho\\rustLang-ws\\rust_exercises\\advent_of_code\\challenge3\\input.txt");
	
	if let Ok( f ) = File::open(path) {
		let mut file = f;
		let mut data = String::new();

		file.read_to_string(&mut data);
		
		
		println!("Hited trees (part one): {}", part_one(&data) );
		println!("Hited trees (part two): {}", part_two(&data) );
	}
}

fn part_one( data : &String ) -> usize {

	let mut pos = 0;
	let mut hited_tree : usize = 0;
	for line in data.lines().skip(1) {
		pos += 3;

		if pos >= line.len() {
			pos %= line.len();
		}

		if &line[pos..(pos+1)] == "#" {
			hited_tree += 1;
		}
	}

	hited_tree
}

fn part_two( data : &String ) -> usize {

	let slopes : [(u8,u8); 5] = [ (1,1), (3,1), (5,1), (7,1), (1,2) ];

	let mut awnser : usize = 1;

	for (posh, posv) in slopes.iter() {


		let mut right : usize = 0;
		let down =  *posv as usize;
		let mut hited_tree : usize = 0;

		for line in data.lines().skip(down).step_by(down) {
			right += *posh as usize;

			if right >= line.len() {
				right %= line.len();
			}

			if &line[right..(right+1)] == "#" {
				hited_tree += 1;
			}
		}

		awnser *= hited_tree;
	}

	awnser
}

