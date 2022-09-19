#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name?");

    let mut name: String = String::new();
    let greeting = "Nice to meet you";

    std::io::stdin()
        .read_line(&mut name)
        .expect("Didn't receive input");

    println!("Hello {}! {}", name.trim_end(), greeting);
}
