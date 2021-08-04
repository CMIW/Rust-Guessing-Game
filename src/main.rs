// This is a simple guessing game, taken from the book "The Rust Programming Language", to learn
// the language trying and practicing common Rust concepts. The program generates a random integer
// between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered,
// the program will indicate whether the guess is too low or tool high. If the guess is correct,
// the game will print a congratulatory message and exit.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Guess the number!");

	// Store a random number from 1 to 100. "gen_range" takes a range expression as an argument and
	// generate a random number in the range. It's inclusive on the lower bound but exclusive on
	// the upper bound.
	let secret_number = rand::thread_rng()
		.gen_range(1..101);

	loop {
		println!("Please input your guess.");

		// The "let" statement is used to create variables. In Rust by default variables inmmutable
		// . So "mut" is used before a variable name to make a variable mutable. This line creates
		// a mutable variable that is bound to a new, empty instance of a "String", used to store
		// the user input.
		let mut guess = String::new();

		// This line calls the stdin (standard input) function from the io (input/output) module.
		io::stdin()
			.read_line(&mut guess)				// Get the input from user and store it on guess.
			.expect("Failed to read line.");	// Crash when a problem occurs and print message.

		// This line takes the value of guess removes the white spaces or newline (\n or \r\n) and
		// parses it to a u32 interger.
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_)	=> continue,
		};

		// This line prints the user's input.
		println!("You guessed: {}", guess);

		// Compares the guess value with the secret number. When it matches the player wins and the
		// game ends.
		match guess.cmp(&secret_number) {
			Ordering::Less 		=> println!("Too small!"),
			Ordering::Greater 	=> println!("Too big!"),
			Ordering::Equal 	=> {
				println!("You win!");
				break;
			}
		}
	}
}
