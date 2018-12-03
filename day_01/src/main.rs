use std::fs::File;
use std::io::prelude::*;

fn open_file() -> String {
  let f = File::open("files/input.txt");
  let mut contents = String::new();
  f.unwrap().read_to_string(&mut contents).expect("Something wen wrong while reading the file");
  
  contents
}

fn part_one() {
  let contents = open_file();
  let arr: Vec<&str> = contents.split_whitespace().collect();
  let i_arr: i32= arr.into_iter().map(|e| e.parse::<i32>().unwrap()).sum();
  println!("{:?}", i_arr);
}

fn part_two() {
  let contents = open_file();
}

fn main() {
  part_one();
}
