use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let path = Path::new("E:\\trabalho\\rustLang-ws\\rust_exercises\\advent_of_code\\challenge2\\input.txt");
	
	if let Ok( f ) = File::open(path) {
		let mut file = f;
		let mut data = String::new();

		file.read_to_string(&mut data );

		println!("Total corrects passwords (part one): {}", part_one(&data));
		println!("Total corrects passwords (part two): {}", part_two(&data));

	}
	
}

fn part_one( data : &String ) -> usize {
	let mut response = 0usize;

	for line in data.lines() {
		let mut ws = line.split_whitespace();

		let mut times = ws.next().unwrap().split("-");
		let minimum : usize = times.next().unwrap().parse().unwrap();
		let maximum : usize = times.next().unwrap().parse().unwrap();
		
		let mut rule = ws.next().unwrap();
		let rule_length = rule.len();
		rule = &rule[0..(rule_length -1)];

		let password = ws.next().unwrap();

		let count : usize = password.matches(rule).count();

	
		if count >= minimum && count <= maximum {
			response += 1;
		}

	}

	response
}

fn part_two( data : &String ) -> usize {
	let mut response = 0usize;

	for line in data.lines() {
		let mut ws = line.split_whitespace();

		let mut times = ws.next().unwrap().split("-");
		let minimum : usize = times.next().unwrap().parse().unwrap();
		let maximum : usize = times.next().unwrap().parse().unwrap();
		
		let mut rule = ws.next().unwrap();
		let rule_length = rule.len();
		rule = &rule[0..(rule_length -1)];

		let password = ws.next().unwrap();

		let first = &password[(minimum-1)..minimum ] == rule;
		let second = &password[(maximum-1)..maximum] == rule;

		if (first && !second ) || (second && !first) {
			response += 1;

			println!("{} -> valid", &line);
		} else {
			println!("{} -> INvalid", &line);
		}
	
	}

	response
}

