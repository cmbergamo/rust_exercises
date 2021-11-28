use std::fs::{OpenOptions, File};
use std::path::Path;
use std::io::{Read};
use std::str::Lines;

fn main() {
    let res_file =  OpenOptions::new()
	    .read(true)
        .open(Path::new("input.txt") );

	match res_file {
		Ok( mut file ) => {
			read_file( &mut file );
		},
		Err( e ) => {
			println!("{}", e);
		}
	}
}

fn read_file( file : &mut File) {
	let mut data = String::new();

	match file.read_to_string( &mut data) {
		Ok( size ) => {
			if size > 0 {
				manipulate_data(&data);
			}
		},
		Err( e ) => println!("{}", e.to_string())
	}
}

fn manipulate_data( data : &String ) {

	let lines = data.lines();
	let mut preamble = [0; 25];
	let mut new_iten= 0;
	let mut id : usize = 0;

	for l in lines.clone() {

		if id >= 25 {
			new_iten = l.parse().unwrap();

			let resp = verify_sum(&preamble, &new_iten );

			if resp {
				change_preamble(&mut preamble, new_iten);
			} else {
				println!( "Item com erro: {}", &new_iten);

				break;
			}
			// println!("{} <=> {}", id, l);
		} else {
			preamble[id] = l.parse().unwrap();
		}

		id += 1;
	}

	find_contigous_set( &lines, &new_iten);

}

fn change_preamble( preamble : &mut [usize], new_iten : usize ) {
	let size = preamble.len();

	for id in 1..size {
		preamble.swap(id - 1, id);
	}

	preamble[24] = new_iten;

}

fn verify_sum( preambule : &[usize], iten : &usize) -> bool {

	let size = preambule.len();
	for id1 in 0..(size -1) {

		for id2 in (id1 + 1)..size {

			let val1 = preambule[id1];
			let val2 = preambule[id2];

			if val1 + val2 == *iten {
				return true;
			}

		}
	}

	false
}

fn find_contigous_set( lines : &Lines, invalid : &usize) {

	let mut total : usize = 0;
	let mut jump : usize = 0;
	let mut minor : usize = 0;
	let mut bigger : usize = 0;

	while total != *invalid {
		for line in lines.clone().skip(jump) {
			let num : usize = line.parse().unwrap();

			total += num;

			if total < *invalid {

				if minor == 0 || num < minor {
					minor = num;
				}

				if num > bigger {
					bigger = num;
				}

			} else if total > *invalid {
				jump += 1;
				total = 0;
				minor = 0;
				bigger = 0;

				break;
			} else {

				if minor == 0 || num < minor {
					minor = num;
				}

				if num > bigger {
					bigger = num;
				}

				break;
			}
		}
	}

	println!("The add of minor and bigger is: {}", ( minor + bigger ) );
}
