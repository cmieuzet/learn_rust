// use std::env;

fn main() {
    for arguments in std::env::args() {
        println!("{arguments}");
    }
    println!("Hello, world!");
}
