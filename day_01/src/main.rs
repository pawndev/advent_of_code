use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

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

fn part_two() -> Result<(), Box<::std::error::Error>>{
  let contents = open_file();
  let mut last_res = 0;
  let mut occ = HashSet::new();
  occ.insert(0);

  loop {
    for entry in contents.lines() {
      let i: i32 = entry.parse()?;
      last_res += i;
      if occ.contains(&last_res) {
        println!("{:?}", last_res);
        return Ok(());
      }

      occ.insert(last_res);
    }
  }
}

fn main() {
  part_one();
  part_two();
}
