mod vehicle;
mod customer;
mod datastructures;
mod consts;

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("wot?");
    println!("h {}", s);
}
