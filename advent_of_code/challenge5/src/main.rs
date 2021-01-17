use std::{path::Path, u16};
use std::fs::File;
use std::io::prelude::*;

fn main() {

    let path = Path::new("./input.txt");

    if let Ok(mut file) = File::open(path) {
        let mut buf = String::new();
        
        if let Ok( _ ) = file.read_to_string(&mut buf) {
            println!("Highest seat ID (part one): {}", part_one(&buf));
            println!("The ID of my seat (part two): {}", part_two( &buf ));
        }
    }
}

fn part_one( data : &String ) -> u16 {

    let mut seat_id = 0u16;

    for line in data.lines() {
        let temp = decode(line);

        if temp > seat_id {
            seat_id = temp;
        }
    }

    seat_id
}

fn part_two( data : &String ) -> u16{
    let mut seats = Vec::new();

    for line in data.lines() {
        seats.push( decode( line ) );
    }

    seats.sort();

	let mut val = 0;

    if let Some( control ) = seats.get(1) {
		val = *control;

		for pos in seats.iter() {

			if (val -1) != *pos {
				println!("Achou: {}", val-1);
				return val - 1;

			}

			val += 1;

		}

    }

    0
}


fn decode( code : &str ) -> u16 {

    let mut row = 0u16;

    for c in code.chars() {
        match c {
            'F' | 'L' => row = row << 1,
            'B' | 'R' => {
                row = (row << 1) + 1;
            },
            _ => ()
        }
    }

    row

}


#[cfg(test)]
mod test {
    use crate::decode;

    #[test]
    fn test_decode_row( ) {

        assert_eq!(decode("BBBBBBB"), 127);
        assert_eq!(decode("FFFFFFF"), 0);
        assert_eq!(decode("FBFBBFF"), 44);
        assert_eq!(decode("BFFFBBF"), 70);
        assert_eq!(decode("FFFBBBF"), 14);
        assert_eq!(decode("BBFFBBF"), 102);
    }

    #[test]
    fn test_decode_column( ) {

        assert_eq!(decode("RLR"), 5);
        assert_eq!(decode("RRR"), 7);
        assert_eq!(decode("RLL"), 4);
        assert_eq!(decode("LLL"), 0);
    }

    #[test]
    fn test_seat_id( ) {
        assert_eq!(decode("BFFFBBFRRR"), 567);
        assert_eq!(decode("FFFBBBFRRR"), 119);
        assert_eq!(decode("BBFFBBFRLL"), 820);
    }
}