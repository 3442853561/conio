#![feature(asm)]

#[macro_use]
extern crate conio;
use conio::*;




#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe fn read_byte_in() -> u8 {
let result: u8;
asm!("mov ah 0");
asm!("int 16h" : "={al}"(result));
result
}
	
fn main() {
	let num_1:i32=numin!();
	let num_2:i64=numin!();
	println!("{}+{}={}",num_1,num_2,(num_1 as i64)+num_2);
	println!("{}",getchar!());
	printf!("ok");
	printf!("ok");
	printf!("ok");
	unsafe{
	printf!("{}",read_byte_in());}
} 