// Exploring structs

//utilize rand_info.rs with "mod"
// (not being used now because)
// (we're using the use statement below)
//mod rand_info;

// and also use the "use" keyword so that you
// won't need to keep referencing with "mod"
use crate::structs::rand_info::*;  //use everything in rand_info

// Structs: like Classes, but not in the sense of i.e. OOP
// no inheritance, but can have methods
// have traits: polymorphism in a sense (like OOP)
// deriving traits with macros

#[allow(dead_code)]
// defining a struct
// instead of inheritance, rust has composition
#[derive(Debug)]
struct MyData {
	// define the fields/members/data elements of the struct
	// Better to organize them alphabetically
	some_bool: bool,
	some_float: f64,
	some_int: i32,
	// composition (instead of inheritance)
	random: RandomInfo,
}

// creating implementation of struct randominfo (using rand_info)
// this shows that you can implement in structs from about anywhere
// and add functionality to structs in whatever way you want
impl RandomInfo {
	pub fn is_larger(&self, compare_to: i64) -> bool {
		self.some_int > compare_to
	}
}


// Example: a trait decouples function definitions
// can be used on multiple structs.
// This is how polymorphism is achieved
// for instance,
impl SomeTrait for MyData {
	fn is_valid(&self) -> bool {
		true
	}
}

//and a function that accepts multiple types
// for traits as function parameters, you need to use "dyn"
fn print_if_is_valid(check_me: &dyn SomeTrait) {
	if check_me.is_valid() {
		println!("yay");
	}
}

// some traits have helpers in the forms of macros
// We will use a predefined macro because implementing
// the Debug trait is tedious

// we could derive more things too, such as
// Copy, Clone, PartialEq, etc. #[derive(Debug)]

// Rust has some predefined traits
// this one is called "Default"
impl Default for MyData {
	fn default() -> Self {
		Self {
			some_bool: true,
			some_float: 10.3,
			some_int: 80,
			random: RandomInfo::new(true),
		}
	}
}

#[allow(unused_variables)]
pub fn main() {
	// utiilize rand_info
	let mut rand_info_var = RandomInfo::new(false);//MyData::default();
	/*RandomInfo {
		// call counts: mutation
		call_count: 0,
		some_bool: true,
		some_int: 10,
	};*/

	// utilize RandomInfo's "compare_to" function
	// the "self" parameter is assumed
	let is_this_smaller = rand_info_var.is_smaller(9);
	// rand_info_var.is_smaller VS rand_info_var::is_smaller
	// dot operator: reading/writing to a variable, use this
	//(lowercase self parameter in function)
	// two colons: a function to be accessible by a Type
	//(no lowercase self parameter in function)

	let is_this_larger = rand_info_var.is_larger(20);
	let is_valid = rand_info_var.is_valid();


	// use mut to allow editing members/fields
	let mut my_var = MyData { 
		some_bool: true,
		some_float: 10.3,
		some_int: 3,
		// define the new member
		// we utilize our newly created function
		// to create an instance
		random: RandomInfo::new(true),
	};

	//update the variable
	my_var.some_int = 100;


	//new variable
	let a_var = MyData {
		// only change one variable
		//the rest will be the same as the above variable
		some_int: 200,
		..my_var
	};


	println!("{:?}", my_var);

	// This function can be called only on any data type
	// that uses some sort of trait
	// both RandomInfo and MyData use SomeTrait so it
	// can be used on both datatypes.
	print_if_is_valid(&rand_info_var);
	print_if_is_valid(&a_var)

}