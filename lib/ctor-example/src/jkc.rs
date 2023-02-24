use ctor::*;

#[ctor]
fn start() {
    println!("start...");
}

#[dtor]
fn end() {
    println!("end...");
}
