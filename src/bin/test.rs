#[macro_use]
extern crate conio;
use conio::*;

fn main() {
	let num_1:i32=numin!();
	let num_2:i64=numin!();
	println!("{}+{}={}",num_1,num_2,(num_1 as i64)+num_2);
	println!("{}",getchar!());
	printf!("ok");
	printf!("ok");
	printf!("ok");
} 