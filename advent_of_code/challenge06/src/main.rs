use std::collections::HashMap;
use std::iter;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let path = Path::new("./input.txt");

	if let Ok( mut file ) = File::open(path) {

		let mut buf = String::new();
		
		if let Ok( _ ) = file.read_to_string( &mut buf ) {

			println!("Total distinct answers (part one): {}", count_answers_anyone( &buf ) );
			println!("Total everyone answers (part two): {}", count_answers_everyone( &buf ) );
		}

	}
	
}

fn count_answers_anyone( data : &str ) -> usize {
	let mut hm : HashMap<char, bool> = HashMap::new();
	let mut sum = 0usize;

	for line in data.lines().chain(iter::once("")) {

		
		if line != "" {
			for c in line.chars() {
				if !c.is_whitespace() {
					hm.entry(c).or_insert(true );
				}
			}
		} else {
			sum += hm.len();

			hm.clear();
		}


	}

	sum
}

fn count_answers_everyone( data : &str ) -> usize {
	let mut hm : HashMap<char, usize> = HashMap::new();
	let mut sum = 0usize;
	let mut count_people = 0usize;

	for line in data.lines().chain(iter::once("")) {
		
		if line != "" {
			for c in line.chars() {
				if !c.is_whitespace() {
					let total = hm.entry(c).or_insert(0 );
					*total += 1;
				}
			}
			count_people += 1;
		} else {
			hm.retain( | _ , v | {
				*v == count_people
			});
			
			sum += hm.len();

			hm.clear();
			count_people = 0;
		}

	}

	sum
}

#[cfg(test)]
mod test {
    use crate::{count_answers_anyone, count_answers_everyone};

	#[test]
	fn test1() {
		let validator =
"abc

a
b
c

ab
ac

a
a
a
a

b";
		assert_eq!(count_answers_anyone(validator), 11);
	}

	#[test]
	fn test2() {
		let validator =
"abc

a
b
c

ab
ac

a
a
a
a

b";
		assert_eq!(count_answers_everyone(validator), 6);
	}

}