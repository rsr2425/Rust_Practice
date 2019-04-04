//
// Climbing Ropes
//
// Simple program to play around with the 'rope'
// data structure. This is also my second program
// in Rust.
//

struct Rope<'a> {
    len: usize,
    left: Option<Box<Rope<'a>>>,
    right: Option<Box<Rope<'a>>>,
    data: &'a String,
}



fn main() {
    println!("Hello, world!");
}
