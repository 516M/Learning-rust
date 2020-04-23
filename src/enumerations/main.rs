// Learning about enumerations

// Example enums
// Note that we are not limited to simple types
// like integers, floats ,etc.
// we can also use strings(tuples) in combination with other types
// or we can even use structs
enum Payment {
	// It is important to consider that
	// when an enum is not being used
	// or when you're first writing your enums,
	// you should put an underscore before their
	// names to indicate that they're not being
	// used. As you go, you will eventually
	// get rid of the underscores yourself,
	// but it's important to understand this
	// indication.

	// For instance, here we include
	// an amount with the Cash item
	Cash(f32),

	// We add another option here,
	// which causes the match event
	// in the main function to get
	// an error due to the fact that
	// the new option, DebitCard,
	// is not covered.
	// This will alert you via an error
	// so you will need to implement
	// this new option (already done).
	CreditCard(String, f32),

	// This enum uses a struct as data.
	DebitCard(DebitData),

	// you can also create a 
	// strongly typed enum item,
	// similar to how you'd create a struct.
	Crypto { account_id: String, amount: f32, },

	// unused example
	_Unuse,
}

// A struct example
// to be used as enum data
struct DebitData {
	pub card_number: String,
	pub amount: f32,
}

pub fn main() {
	// here we will define one of each enum
	// that we defined above
	// We do this for the sake of showing
	// the examples.

	// nothing big in particular here
	// note that Cash only accepts floats (f32).
	let cash_payment = Payment::Cash(59.02);
	process_payment(cash_payment);

	// use ".to_string()" to convert
	// sentences surrounded by quotes into
	// strings.
	let cc_payment = Payment::CreditCard("shitcard".to_string(), 39.02);
	process_payment(cc_payment);

	// since debit card accepts structs,
	// we also have to consider this fact
	// and thus we define the DebitCard parameter
	// by putting an entire definition of DebitData
	// struct within (with all the required values)
	let dc_payment = Payment::DebitCard(DebitData {
		// members/fields followed by semicolon
		// and what we want their values to be.
		card_number: "Debit Num".to_string(),
		amount: 255.53
	});
	process_payment(dc_payment);

	// crypto was defined above as
	// a strongly typed enum, 
	// so we have to create an instance of
	// it as such:
	let crypto_payment = Payment::Crypto{account_id: "ABC 123".to_string(), amount: 999.03};
	process_payment(crypto_payment);

	// unused enum
	let unused_var = Payment::_Unuse;
	process_payment(unused_var);
}

// Process payment:
// accepts a Payment enum and
// returns a response (message)
// parameters: some_payment: a Payment enum
fn process_payment(some_payment: Payment) {
	// we perform a "switch" case
	// on the given variable

	// Note: it is important that
	// the parameters specified inside the parentheses
	// of the Enums have the exact same names as
	// the parameters of the enums defined at the start of this file
	match some_payment {
		// In here we define all the enums
		// which we will be looking at
		// in order to satisfy the match case.

		//amt: amount; As defined in the enums,
		// it accepts f32 (float) variables.
		Payment::Cash(amt) => {
			println!("Paying with Cash... in the amount of {}", amt);
		}
		
		// some_string, some_f32:
		// as defined in the enums, it accepts a String
		// and f32 (float) variables.
		// Unused variables can either have an underscore in front
		// or be renamed into an underscore
		Payment::CreditCard(some_string, _some_f32) => {
			println!("Paying with Credit card.. some_string {}", some_string);
		}

		// data: as defined above in the enums,
		// this enum accepts a struct. look at the struct
		// above to understand what members/fields it has.
		// those members/fields are used below
		// in order to be printed out.
		Payment::DebitCard(data) => {
			println!("Paying with debit card.. card_number {}, amount {}", data.card_number, data.amount);
		}

		// since we defined the crypto enum 
		// similarly to how structs are defined,
		// we need to put curly brackets around it
		// and follow the rest of the data normally
		Payment::Crypto{account_id, amount} => {
			println!("Paying with cryptocurrency.. amount_id {} amount {}", account_id, amount);
		}
		// If you want to omit an item from being checked,
		// you can use the underscore, which can
		// act as the rest of the case scenarios
		// for which something is defined.
		//_ => ()
		_ => {
			println!("Invalid payment method");
		}
	}
}