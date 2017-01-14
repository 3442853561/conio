pub use std::io::{self, Write};
#[macro_export]
macro_rules! numin {
	() =>{
	{let mut input = String::new();
	std::io::stdin()
	    .read_line(&mut input)
        .expect("Failed to read line");
	input.trim().parse().unwrap()}
	};
}

#[macro_export]
macro_rules! getchar {
	() =>{
	{let mut input = String::new();
	std::io::stdin()
	    .read_line(&mut input)
        .expect("Failed to read line");
	input.chars().nth(0).unwrap()}
	};
}

#[macro_export]
macro_rules! printf {
	($($arg:tt)*) =>{
		print!($($arg)*);
		io::stdout().flush().unwrap();
	}
}


