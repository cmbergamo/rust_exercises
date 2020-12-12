fn main() {
	let s = String::from("book");
	
	// Add code here that calls the pluralize function
	println!( "I have one {}, you have two {}",
		s,
		pluralize(s.clone())
	);

	// Another option withou function
	println!( "I have one {0}, you have two {0}s",
		s
	);

	let mut plural = s.clone();
	pluralize_withou_return(&mut plural );

	println!( "I have one {}, you have two {}",
		s,
		plural
	);
}

// Add apropriate parameters, return values, and implementation to this function
fn pluralize( mut origin : String ) -> String {
	origin.push('s');
	origin
}

// Add apropriate parameters, return values, and implementation to this function
fn pluralize_withou_return( origin : &mut String ) {
	origin.push('s');
	//origin
}