// RandomInfo struct, used in main.rs

// This is like OOP's abstract methods
// Create a trait (polymorphism)
pub trait SomeTrait {
	// what should be implemented if a struct
	// wants to utilize a trait

	// &self: actual data (internally)
	// you don't need to put pub, it's implied to be visible
	fn is_valid(&self) -> bool;

	//fn get_the_better_one(&self, some_other_dude: &Self) -> Self;
}

#[derive(Debug, Clone, Copy)]
// In order for this struct to be used,
// you need the "pub" (public) keyword
// to allow visibility (and/or useability) of the STRUCT
pub struct RandomInfo {
	// we strictly need to specify
	// which fields are visible

	// internal updates on data (AKA mutations)
	// keep track of how many times a function is called
	pub call_count: i64,

	pub some_bool: bool,
	pub some_int: i64,
}

// implement the trait for RandomInfo struct
impl SomeTrait for RandomInfo {
	fn is_valid(&self) -> bool {
		self.some_bool
	}
}

// use impl to create the functions/implement the structure
impl RandomInfo {
	//"pub" to make it visible
	// -> RandomInfo: return a randominfo struct
	// but we can also use Self, to return the type of the implementation
	pub fn new(param_a: bool) -> Self { //RandomInfo {
		// utilize the Self keyword
		// it will refer to the same type of the struct
		Self {
			call_count: 0,
			some_bool: !param_a, //put the opposite of what is given?
			some_int: 8,
		} //no semicolon because this is what we return
	}

   // &Self : type
   // &self : actual data (internally)

   // this function checks two values and returns a bool value
	pub fn is_smaller(&mut self, compare_to: i64) -> bool {
		// raise call count by 1
		// if a variable is assigned this struct,
		// it must be mutable
		self.call_count += 1;

		// compare the struct's "some_int" to the given integer
		self.some_int < compare_to
	}
}