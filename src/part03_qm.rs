use std::{io::prelude::*, ptr::NonNull};
use std::io;

fn read_vec() -> Vec<i32> {
  let mut vec: Vec<i32> = Vec::<i32>::new();
  let stdin = io::stdin();
  println!("Enter a list of numbers, one per line. End with Ctrl+D.");
  for line in stdin.lock().lines() {
    let line = line.unwrap();
    match line.trim().parse::<i32>() {
      Ok(num) => {
        vec.push(num)
      },
      Err(_) => {
        println!("Error!Stdin should be number.")
      }
    }
  }
  vec
}

use part02_qm::{SomethingOrNothing,Something,Nothing,vec_min};

pub fn main() {
  let vec = read_vec();
  let min = vec_min(vec);
  min.print();
}