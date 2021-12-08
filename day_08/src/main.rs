use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "test.txt";

fn main() {
  println!("Advent of Code - Day 01");
  let input = read_input(FILE_NAME);

  // for r in &input {
  //   println!("Reading: {}", r);
  // }

  println!("----------------------");
  println!("Starting Puzzle 1");
  let result = puzzle_1(&input);
  println!("Total count: {}", result);
  println!("----------------------");
  println!("Starting Puzzle 2");
  let result = puzzle_2(&input);
  println!("Total count: {}", result);

}

fn read_input(filename: &str) -> Vec<String> {

  let mut readings: Vec<String> = Vec::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    readings.push(line.to_string())
  }

  readings

}

fn puzzle_1(readings: &Vec<String>) -> i64 {
  // let readings = readings.to_vec(); // Create a copy of Readings
  let mut count: i64 = 0;

  for val in readings {
    let part: Vec<&str> = val.split("|").collect();
    let parts: Vec<&str> = part[1].split(" ").collect();
    for p in parts {
      let l: usize = p.len();
      if l == 2 || l == 4 || l == 3 || l == 7 {
        count += 1
      }
    }
  }
  count
}

fn puzzle_2(readings: &Vec<String>) -> i64 {
  let mut count: i64 = 0;

  for val in readings {
    let parts: Vec<&str> = val.split("|").collect();
    let nums: Vec<&str> = parts[0].split(" ").collect();
    
  }

  count += 1;

  count
}

