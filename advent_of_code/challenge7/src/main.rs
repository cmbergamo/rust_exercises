use std::{borrow::{Borrow, BorrowMut}, io::prelude::*, u32, usize};
use std::vec::Vec;
use std::fs::File;
use std::path::Path;

fn main() {
	let path = Path::new("input.txt");
	if let Ok( f ) = File::open(path) {
		let mut buf = String::new();

		let mut file = f;
		if let Ok( _ ) = file.read_to_string( &mut buf) {
			println!( "Bag colors that can handle shiny gold (part one): {}", part_one( &buf ) );
			println!("Required individual bag inside shiny gold (part two): {}", part_two( &buf) );
		}
	}
}

fn bag_finder( data : &str, bag : &str ) -> Vec<String> {
	let mut vec = Vec::<String>::new();
	for line in data.lines() {
		
		let search_dat = line.split("contain" ).collect::<Vec<&str>>();

		if search_dat.len() > 1 {

			let bags = search_dat[1].split(", ").collect::<Vec<&str>>();
			
			for b in bags.iter() {
				
				if b.contains(bag) {
					let mut bag_temp = search_dat[0].trim();
					
					if let Some( index ) = bag_temp.find(" bag") {
						bag_temp = &bag_temp[..index];
					}
					
					if !vec.contains( &bag_temp.to_owned() ) {
						vec.push(bag_temp.to_owned());
					}
					
					let vec_temp = bag_finder(data, bag_temp);
					
					for vt in vec_temp {
						
						if !vec.contains( &vt ) {
							vec.push( vt );
						}

					}

				}
				
			}

		}
	}

	vec
	
}

fn bag_counter( data : &str, bag : &str ) -> Option< Box< Bag > > {

	for line in data.lines() {

		let line_split = line.split(" contain ").collect::< Vec< &str> >();

		if line_split[0].contains( &bag ) {

			let mut key = line_split[0];

			if let Some( index) = key.find(" bag") {
				key = &key[..index];
			}

			let mut bag = Box::new(Bag::new( key.to_owned() ) );

			let insiders = line_split[1].split(", ").collect::< Vec< &str> >();

			if insiders.len() > 1 {
				
				for ins in insiders {

					create_bag(&data, ins, bag.borrow_mut())

				}
			} else if !insiders[0].contains("no other") {

				create_bag(&data, insiders[0] , bag.borrow_mut());
			}

			return Some( bag );
		}

	}

	None

}

fn parse_into_bags( desc : &str ) -> Option< ( String, u32 ) >{

	if let Some( index ) = desc.find(" ") {
		let idx = desc[..index].parse::<u32>().unwrap_or(0);
		let mut name = &desc[ (index + 1) .. ];

		if let Some( bag_index ) = name.find(" bag") {
			name = &name[ .. bag_index];
		}

		return Some( ( name.to_owned(), idx ) );
	}

	None
}

fn create_bag( data : &str,  key : &str, bag : &mut Box<Bag> ) {
	if let Some( opt ) = parse_into_bags( key ) {
		let ( n, i ) = opt;

		if let Some( bi ) = bag_counter( &data, &n) {
			bag.add( bi, i);
		}

	}
}

fn bag_sum( path : &Box<Bag> ) -> usize {
	let mut sum = 0;
	
	for p in &path.list {
		let (b, i ) = p;
		let wheight = *i as usize;
		sum += wheight + wheight * bag_sum(b.borrow() );

	}

	sum

}

fn part_one( data : &str ) -> usize {
	let vec = bag_finder(data, "shiny gold" );

	vec.len()
}

fn part_two( data : &str ) -> usize {
	let bags = bag_counter(&data, "shiny gold" );

	if let Some( b ) = bags {
		return bag_sum(b.borrow());
	}

	0
}

#[derive(Debug)]
struct Bag {
	name : String,
	list : Vec< (Box < Bag >, u32) >
}

impl Bag {

	fn new( name : String ) -> Bag {
		Bag{
			name: name,
			list: Vec::new()
		}
	}

	fn add(&mut self,  b : Box<Bag>, i : u32) {
		self.list.push( (b, i) );
	}
}

#[cfg(test)]
mod test {
    use crate::{bag_counter, bag_finder, part_two};

	fn return_data() -> String {
		let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.
	dark orange bags contain 3 bright white bags, 4 muted yellow bags.
	bright white bags contain 1 shiny gold bag.
	muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
	shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
	dark olive bags contain 3 faded blue bags, 4 dotted black bags.
	vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
	faded blue bags contain no other bags.
	dotted black bags contain no other bags.
	test".to_owned();
	
		data
	}

	fn return_data_2() -> String {
		return "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.".to_owned();
	}

	#[test]
	fn test_find_bags() {
		let demo = return_data();

		let resp = bag_finder(&demo, "shiny gold");

		assert_eq!(resp.len(), 4);
	}

	#[test]
	fn test_cout_bags() {
		{
			let demo = return_data();

			assert_eq!( part_two(&demo ), 32);
		}

		{
			let demo = return_data_2();

			assert_eq!( part_two(&demo ), 126);
		}

	}
}
