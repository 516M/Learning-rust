// Primitives in Rust

// the below code is put in place
// so that the compiler doesn't give us
// any warnings (for the sake of these examples).
// Allow unused variables for the sake of examples
#[allow(unused_variables)]
// Allow unused assignments for the sake of examples
#[allow(unused_assignments)]
// Allow non snake case variables
#[allow(non_snake_case)]
// Allow unused mutable variables
#[allow(unused_mut)]

pub fn main() {
	// the 'let' keyword: 
	// create a variable and assign
	// data to it. the compiler
	// itself will be able to recognize
	// the type of the variable even
	// if you didn't assign a type to it.

	// it is preferable to use
	// snake case when creating variables.

	// a boolean primitive
 	let mut some_data: bool = true;//or false
	//let some_data: bool = true;
	//some_data = false;
	//          ^^^^^^^
	// taking into account only the
	// two commented out lines of code above,
	// it will be impossible to change the value
	// of this variable, because
	// it is NOT mutable(therefore, it's immutable).
	// what this means is that by being immutable,
	// a variable's value cannot be changed.
	// you could say it's kinda like a constant.
	// if you want to make a variable mutable,
	// all you have to do is use the 'mut' keyword
	// right after the 'let' keyword. 
	// i.e. 'let mut data: bool = true;
	// you will be able to change the value of the above variable.

	//a primitive integer
	// i8: stands for "integer 8-bit numbers";
	// this means that the variable will
	// be able to hold whole
	// numbers which if counted, should amount
	// to be 2^8 numbers in total.
	// as you should have guessed, there are
	// other integer types: i16, i32, i64, etc;
	// they can all hold numbers
	// which in total, should amount
	// to be 2 to the power of a certain number
	// (whether it's 8, 16, 32, etc.)
	// now, note that the fact that you're using
	// i8 (or any of the above integer types)
	// means that you're also handling negative
	// integers.
	// if you try to assign a number that goes
	// out of the range of the given type,
	// the compiler will give an error.
	// i.e. let some_data: i8 = 200;
	//                        ^^^^^
	// the value is outside the maximum range
	// of the type i8.
	let some_data: i8 = 10;

	// you can use the standard type definition
	// (std) to check the MAX and MIN values
	// that these types can take.
	// for instance, check the code below:
	println!("Min i8 is {}", std::i8::MIN);
	println!("Max i8 is {}", std::i8::MAX);
	// (this will print -127 and 128 respectively.)
	// NOTE: a very important issue in programming that
	// you'll encounter:
	// say you'd be adding 2 values for
	// the type i8: 120 and 10; This
	// would give us 130. However,
	// the compiler would NOT cover
	// this case, and if this is left
	// uncovered, the program WILL 
	// crash, because
	// the value 130 is beyond the
	// range of the 8 bit integer type (i8).

	// running the code below in debug
	// will cause panic (essentially a crash
	// but you do get that the reason is
	// due to an attempted overflow.)
	//let more_data: i8 = some_data + 120;

	// NOTE: BUILDING IN RELEASE MODE
	// WILL CAUSE THE VALUE TO OVERFLOW
	// AND WRAP AROUND FROM THE MIN VALUE.
	// be sure you're using the correct data
	// type, since release and debug modes
	// are very different, and all things
	// can go downhill if you aren't clear
	// about what you're trying to program.
	// if you know that you'll be using big numbers,
	// use a bigger integer bit number.

	// in situations where you don't have to worry
	// about negative numbers, you can use
	// unsigned integers; only positive integers.
	// so if we took the type u8, for instance,
	// (u8 stands for unsigned integer 8 bit number)
	// unlike i8, whose range was from -127
	// up to 128, the range for u8 doubles,
	// becoming 128*2 = 256 of total numbers.
	// the range will be 0 to 255. remember that
	// a value is reserved for 0, meaning
	// that you cannot get 256.

	//let some_data: u8 = -10; // from 0 to 255
	// note that if you assign negative
	// numbers, they will be caught by the compiler.

	//like the previous type (i8), remember that
	//overflows can occur, and it is important
	// that you make sure to be clear about
	// what you want to do with your code.
	let some_other_data = 255 + 2;//error;too big
	let some_other_data2 = 1 - some_data; //error;too small

	// for larger numbers, there are the
	// u16(16 bit), u32(32 bit), u64(64 bit)
	// and u128(bit) types.
	// Note that this also applies to several
	// other types, such as the integers
	// (i.e. i128, where you have both negative
	// and positive numbers).

	// i128: from -WOW to +WOW-1 (where WOW is a very large number)
	let big_data: i128 = 10;
	// the number range is pretty big,
	// below the largest/smallest numbers
	// will be printed in console.
	println!("Min i128 is {}", std::i128::MIN);
	println!("Max i128 is {}", std::i128::MAX);
	// most of the time you want to use i32 or i64.
	// if you do not specify integer type, it will be assumed
	// to be an i32.

	// the cpu of some computers can be
	// either 32bit or 64bits. if you want
	// to assign a variable based
	// on that fact, you can use isize;
	// isize will assign either 32bit
	// or 64bit, depending on the architecture
	// of your computer.

	// again, depending on whether your computer
	// is 32bit or 64bit: isize (integer-sized)
	let some_isize: isize = 10;

	// same as above, except unsigned
	let some_usize: usize = 10;

	// now, when we'll want to represent
	// variables with floating decimal points,
	// we utilize the float type.

}
