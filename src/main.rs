// Learning Rust
// (watching the Doug Milford videos)
// https://www.youtube.com/channel/UCmBgC0JN41HjyjAXfkdkp-Q/videos

// For utilizing struct examples
//#[allow(unused_imports)]
//mod structs;
//use crate::structs::main::main as structs;

// For utilizing println! and format! macros examples
//#[allow(unused_imports)]
//mod some_macros;
//use crate::some_macros::main::main as some_macros; 

// For enumeration examples
//#[allow(unused_imports)]
//mod enumerations;
//use crate::enumerations::main::main as enumerations;

// For primitives (basic data) examples
//#[allow(unused_imports)]
mod primitives;
use crate::primitives::main::main as primitives;

// main function
fn main() {
	// Structs
	//structs();

	// some macros
	//some_macros();

	// enumerations
	//enumerations();

	// Primitives
	primitives();
}