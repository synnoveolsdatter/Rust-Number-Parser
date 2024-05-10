use std::io::{self, BufRead};
use crate::Getter;

fn main() {
    let stdin = io::stdin();
    println!("Please enter a word.");
    for l in stdin.lock().lines() {
        println!("{}", Getter::get_nums(&l));
    }
}

