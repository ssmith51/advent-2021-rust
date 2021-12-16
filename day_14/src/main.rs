use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

fn main() {
  println!("Advent of Code - Day 12");
  println!("Starting Puzzle 1");
  let start = Instant::now();
  let duration = start.elapsed();
  println!("Total Duration: {:?}", duration);

}



fn read_input(filename: &str) {
  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
  }
}