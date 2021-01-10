extern crate regex;

use std::{fmt::Display, fs::File};
use std::io::prelude::*;
use std::path::Path;
use std::iter;
use regex::Regex;

fn main() {
	let path = Path::new("./input.txt");
	

	match File::open(path) {

		Ok( mut file ) =>  {
			
			let mut data = String::new();
			if let Ok(_) = file.read_to_string(&mut data) {
				let mut passports : Vec<Passport> = Vec::new();
				println!("Valid passports (part one): {}", part_one(&data, &mut passports));

				println!("Valid passports (part two): {}", part_two( &passports ));
			}

		},
		Err( erro ) => {
			println!("{}", erro)
		}

	}
}

fn part_one( data : &String, buf : &mut Vec<Passport> ) -> u32 {
				
		let mut total_valid = 0u32;
		let mut pp = Passport::new();
		let mut posicao = 1u32;

		for line in data.lines().chain(iter::once("")) {

			if line == "" {

				if pp.is_valid() {
					total_valid += 1;
				}

				posicao += 1;

				buf.push(pp);

				pp = Passport::new();
			} else {

				for fields in line.split_whitespace() {

					let f : Vec<&str> = fields.split(":").collect();

					if let Some(f_value) = f.get(1) {
						match f.get(0) {
							Some( tp ) => {
								match *tp {
									"byr" => pp.byr = {
										if let Ok( value ) = (*f_value).parse::<u16>() {
											Some(value)
										} else {
											None
										}
									},
									"iyr" => pp.iyr = {
										if let Ok( value ) = (*f_value).parse::<u16>() {
											Some(value)
										} else {
											None
										}
									},
									"eyr" => pp.eyr = {
										if let Ok( value ) = (*f_value).parse::<u16>() {
											Some(value)
										} else {
											None
										}
									},
									"hgt" => pp.hgt = {
										if (*f_value) != "" {
											Some((*f_value).to_owned())
										} else {
											None
										}
									},
									"hcl" => pp.hcl = {
										if (*f_value) != "" {
											Some((*f_value).to_owned())
										} else {
											None
										}
									},
									"ecl" => pp.ecl = {
										if (*f_value) != "" {
											Some((*f_value).to_owned())
										} else {
											None
										}
									},
									"pid" => pp.pid = {
										
										if (*f_value) != "" {
											Some((*f_value).to_owned())
										} else {
											None
										}
									},
									"cid" => pp.cid = {
										if let Ok( value ) = (*f_value).parse::<u16>() {
											Some(value)
										} else {
											None
										}
									},
									_ => ()
								}
							},
							None => ()
						}
					}

				}

			}
		}

		println!("Identified passpors: {}", posicao);
		
		total_valid
}

fn part_two( passports : &Vec<Passport> ) -> u32 {

	let mut valids = 0;


	for pp in passports {
		if pp.is_valid_with_data() {
			valids += 1;
		}
	}

	valids
}

#[derive(Debug)]
struct Passport {

	byr : Option<u16>,
	iyr : Option<u16>,
	eyr : Option<u16>,
	hgt : Option<String>,
	hcl : Option<String>,
	ecl : Option<String>,
	pid : Option<String>,
	cid : Option<u16>

}

impl Passport {

	fn new() -> Passport {
		Passport{
			byr : None,
			iyr : None,
			eyr : None,
			hgt : None,
			hcl : None,
			ecl : None,
			pid : None,
			cid : None
		}
	}

	fn is_valid( &self ) -> bool {
		if None ==  self.byr ||
			None == self.iyr ||
			None == self.eyr ||
			None == self.hgt ||
			None == self.hcl ||
			None == self.ecl ||
			None == self.pid {
			false
		} else {
			true
		}

	}

	fn is_valid_with_data( &self ) -> bool {

		let valid_byr = if  let Some( val ) = &self.byr {
			if *val >= 1920 && *val <= 2002 {
				true
			} else {
				false
			}
		} else {
			false
		};

		let valid_iyr = if let Some( iyr ) = &self.iyr {
			if  *iyr >= 2010 && *iyr <= 2020 {
				true
			} else {
				false
			}
		} else {
			false
		};

		let valid_eyr = if let Some( eyr ) = &self.eyr {
			if  *eyr >= 2020 && *eyr <= 2030 {
				true
			} else {
				false
			}
		} else {
			false
		};

		let valid_hgt = if let Some( hgt ) = &self.hgt {
			let slice_hgt = &hgt[hgt.len()-2..hgt.len()];
			if let Ok( height ) = &hgt[0..hgt.len()-2].parse::<u8>() {
				if slice_hgt == "cm" {
					if *height >= 150u8 && *height <= 193u8 {
						true
					} else {
						false
					}
				} else if slice_hgt == "in" {
					if *height >= 59u8 && *height <= 76u8 {
						true
					} else {
						false
					}
				} else {
					false
				}
			} else {
				false
			}
		} else {
			false
		};

		let valid_hcl = if let Some( hcl ) = &self.hcl {

			let rg = Regex::new(r"#{1}[0-9a-fA-f]{6}$").unwrap();
			
			rg.is_match(hcl.as_str() )

		} else {
			false
		};

		let valid_ecl = if let Some( ecl ) = &self.ecl {
			match ecl.as_str() {
				"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
					true
				},
				_ => false
			}
		} else {
			false
		};

		let valid_pid = if let Some( pid ) = &self.pid {
			let rg = Regex::new(r"^[0-9]{9}$").unwrap();

			rg.is_match(pid.as_str() )

		} else { 
			false
		};

		let resp = valid_byr && valid_ecl && valid_eyr && valid_hcl && valid_hgt && valid_iyr && valid_pid;

		if resp {
			println!("{}", &self.to_json());
		}

		resp
	}

	fn count_fields( &self ) -> u8 {
		let mut total = 0u8;

		if let Some(_) = self.byr {
			total += 1;
		}

		if let Some(_) = self.iyr {
			total += 1;
		}

		if let Some(_) = self.eyr {
			total += 1;
		}

		if let Some(_) = self.cid {
			total += 1;
		}

		if let Some(_) = self.hgt {
			total += 1;
		}

		if let Some(_) = self.hcl {
			total += 1;
		}

		if let Some(_) = self.ecl {
			total += 1;
		}

		if let Some(_) = self.pid {
			total += 1;
		}


		total
	}

	fn to_json(&self) -> String {

		String::from(format!("{}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", {}",
			self.byr.unwrap_or(0),
			self.iyr.unwrap_or(0),
			self.eyr.unwrap_or(0),
			if let Some( ref val ) = self.hgt {
				val.as_str()
			} else {
				"NULL"
			},
			if let Some( ref val ) = self.hcl {
				val.as_str()
			} else {
				"NULL"
			},
			if let Some( ref val ) = self.ecl {
				val.as_str()
			} else {
				"NULL"
			},
			if let Some( ref val ) = self.pid {
				val.as_str()
			} else {
				"NULL"
			},
			self.cid.unwrap_or(0)))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test1() {
		let data =
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

		let mut passports : Vec<Passport> = Vec::new();

		part_one( &data.to_owned(), &mut passports);
		
		assert_eq!(8, passports.len());

		assert_eq!( 4, part_two( &passports ) );

	}

}