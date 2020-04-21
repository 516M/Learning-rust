// Exploring macros: println!, format!, etc.

// a structure we'll use
// derive Debug in order to utilize
// std::fmt::Debug trait
#[derive(Debug)]
struct MyData {
	pub a: i32,
	pub b: f32,
}

pub fn main() {
	macro_examples();
	println!();
	printed_examples();
}

// Some macro examples
fn macro_examples() {
	let data = MyData {
		a: 1,
		b: 1.1,
	};

	let other_data = MyData {
		a : 2,
		b: 2.2,
	};

	// {:?} -> This is used because our
	// structure derives the Debug trait
	// which includes std::fmt::Display
	// effectively this will print
	// the strcture inline.
	//println!("My Data is {:?}", data);
	//newline
	//println!();

	// sometimes inline printing is too ugly,
	// and you want complex data to be displayed
	// in better fashion, you can do that
	// with the following, which will
	// format the data better:

	// we force order with ordinal:
	//println!("My data is {1:#?} and {0:#?}", data, other_data);
	
	// we might wanna format a string other than printing to terminal
	// we can use the format statement
	// this will save the output in a variable
	let some_formatted_string = format!("My data is {0:#?} and {1:#?}", data, other_data);

	// we can verify how it looks by printing it
	println!("{}", some_formatted_string);

	//println!("MyData is {}", data);
	// this is errorreous because
	// the structure doesn't implement the trait
	// std::fmt::Display (used to show data).
	// you can fix this by 
	 //1. doing it manually by implementing the display trait
	 //2. use a common shortcut: use #[derive(..)]

	 // the exclamation marks in front of statements
	 // like "format!" or "println!" etc.
	 // they indicate it's a macro, there are different types of macros.
	 // you can create your own macros as well
	 // remember that exclamation marks indicate macros.
}

// Some println examples
#[allow(dead_code)]
fn printed_examples() {
	// print in new line
	println!("Hi");

	// print, no new line
	print!("seperate line");
	//newline
	println!();

	//printing data
	//default: everything is taken with normal order
	let more_data = 6.7;
	println!("Data to print: {} and {}", 5, more_data);

	//printing data in different orders
	// first more_data is printed, after, 5 is printed
	println!("Same data printed in different order: {1}, and {0}", 5, more_data);

	//more strict: using named variables
	println!("Name and surname: {first_name}, {surname}", first_name = "a_name", surname = "a_surname");
}
