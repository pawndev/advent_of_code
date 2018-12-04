use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn open_file() -> String {
  let f = File::open("files/input.txt");
  let mut contents = String::new();
  f.unwrap().read_to_string(&mut contents).expect("Something went wrong while readin the file");

  contents
}

fn part_one() {
  let contents = open_file();
  let mut twos = 0;
  let mut threes = 0;

  for line in contents.lines() {
    let mut occ = BTreeMap::new();
    let mut already_two = false;
    let mut already_three = false;

    for l in line.chars() {
      *occ.entry(l).or_insert(0) += 1
    }

    for (_k, v) in &occ {
      match v {
        3 => if already_three {()} else {threes += 1; already_three = true},
        2 => if already_two {()} else {twos += 1; already_two = true},
        _ => ()
      }
    }
  }
  println!("twos: {:?}, threes: {:?}, twos*threes={:?}", twos, threes, twos * threes);
}

fn main() {
  part_one();
}
