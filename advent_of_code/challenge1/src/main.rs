use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let path = Path::new("E:\\trabalho\\rustLang-ws\\rust_exercises\\advent_of_code\\challenge1\\input.txt");

	if let Ok( file ) = File::open(path) {
		let mut f = file;

		let mut data = String::new();
		f.read_to_string(&mut data);

		println!("The result of part one is: {}", part_one(&data) );
		println!("The result of part two is: {}", part_two(&data) );
	}

}

fn part_one( data : &String) -> u32 {
	let mut resultado : u32 = 0;

	'laco_externo : for (line, val) in data.lines().enumerate() {

		let num1 : u32 = val.parse().unwrap();
		
		for aux in data.lines().skip(line + 1) {

			let num2 : u32 = aux.parse().unwrap();

			if (num1 + num2) == 2020 {
				resultado = num1 * num2;

				break 'laco_externo;
			}
		}
	}

	resultado
}

fn part_two( data : &String) -> u32 {
	let mut resultado : u32 = 0;

	'laco_externo : for (line, val) in data.lines().enumerate() {

		let num1 : u32 = val.parse().unwrap();
		
		for aux in data.lines().skip(line + 1) {

			let num2 : u32 = aux.parse().unwrap();

			for aux2 in data.lines().skip(line + 2) {

				let num3 : u32 = aux2.parse().unwrap();

				if (num1 + num2 + num3) == 2020 {
					resultado = num1 * num2 * num3;

					break 'laco_externo;
				}
			}
		}
	}

	resultado
}
