use std::{fs::File, usize};
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let path= Path::new("input.txt");

	if let Ok( mut file ) = File::open(path) {

		let mut buf = String::new();
		if let Ok( _ ) = file.read_to_string(&mut buf) {

			let lines_original : Vec<&str> = buf.lines().collect();

			println!("Acc value is (part one): {}", acc_until_loop(&lines_original));

			if let Some( resp ) = change_instructions( &lines_original, true ) {
				println!("Acc after correcting instructions (part two): {:?}", resp);
			} else {
				println!("Was not possible to find one solution")
			}
			
		}

	}
	
}

fn acc_until_loop( lines : &Vec<&str> ) -> isize {
	let mut acc = 0isize;
	let mut index = 0usize;
	let mut indexes_readed = Vec::new();

	loop {
		if let Some( line ) = lines.get( index ) {
			let instructions : Vec<&str> = line.split(' ').collect();

			let signal = &instructions[1][0..1];
			let value = &instructions[1][1..];

			if !indexes_readed.contains(&index) {
				indexes_readed.push(index);

				match instructions[0] {
					"acc" => {
						if signal == "+" {
							acc += value.parse::<isize>().unwrap();
						} else if signal == "-" {
							acc -= value.parse::<isize>().unwrap();
						}

						index += 1;
					},
					"jmp" => {
						if signal == "+" {
							index += value.parse::<usize>().unwrap();
						} else if signal == "-" {
							index -= value.parse::<usize>().unwrap();
						}
					},
					"nop" => {
						index += 1;
					}
					_ => ()
				}
			} else {
				break;
			}
		} else {
			break;
		}
	}

	acc
}

fn change_instructions( lines : &Vec<&str>, is_original : bool ) -> Option<isize> {

	let mut acc = 0isize;
	let mut index = 0usize;
	let mut indexes_readed = Vec::new();

	loop {
		if let Some( line ) = lines.get( index ) {
			let instructions : Vec<&str> = line.split(' ').collect();

			let signal = &instructions[1][0..1];
			let value = &instructions[1][1..];

			if !indexes_readed.contains(&index) {
				indexes_readed.push(index);

				match instructions[0] {
					"acc" => {
						if signal == "+" {
							acc += value.parse::<isize>().unwrap();
						} else if signal == "-" {
							acc -= value.parse::<isize>().unwrap();
						}

						index += 1;
					},
					"jmp" => {

						if is_original {
							let mut data2 = lines.clone();
							let new_line = &format!("nop {}", instructions[1]);	

							if let Some( val ) = data2.get_mut(index) {
								*val = new_line;
							}
							
							if let Some ( res ) = change_instructions(&data2, false ) {
								return Some( res );
							}

						}

						if signal == "+" {
							index += value.parse::<usize>().unwrap();
						} else if signal == "-" {
							index -= value.parse::<usize>().unwrap();
						}
					},
					"nop" => {
						if is_original {
							let mut data2 = lines.to_vec();
							let new_line = &format!("jmp {}", instructions[1]);	
							
							if let Some( val ) = data2.get_mut(index) {
								*val = new_line;
							}
							
							if let Some ( res ) = change_instructions(&data2, false ) {
								return Some( res );
							}
							
						}

						index += 1;
					}
					_ => ()
				}
			} else {
				return None;
			}
		} else {
			break;
		}
	}

	Some( acc )
}

#[cfg(test)]
mod test {
    use crate::{acc_until_loop, change_instructions};


	fn return_data_test() -> String {
		String::from("nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6")
	}

	#[test]
	fn test_acc() {
		let data  = return_data_test();
		let lines : Vec<&str> = data.lines().collect();

		let resp = acc_until_loop( &lines );

		assert_eq!(resp, 5)
	}

	#[test]
	fn test_correction() {
		let data  = return_data_test();
		let lines : Vec<&str> = data.lines().collect();

		if let Some( acc ) =  change_instructions( &lines, true ) {
			assert_eq!( acc, 8 );
		} else {
			assert!(false);
		}

	}
}
