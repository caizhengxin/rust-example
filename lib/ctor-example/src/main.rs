use ctor::*;
use ctor_example::jkc::*;

#[ctor]
fn start() {
    println!("start...");
}

#[dtor]
fn end() {
    println!("end...");
}

fn main() {
    println!("loading...");
}
