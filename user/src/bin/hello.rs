#![no_std]
#![no_main]

#[macro_use]
extern crate dcore_user;

#[allow(dead_code)]
#[no_mangle]
fn main() -> i32 {
    println!("Hello world from user main.");
    0
}
