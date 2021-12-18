use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};

//Gobal file name for quick change
const FILE_NAME: &str = "test.txt";

fn main() {
  println!("Advent of Code - Day 12");
  let input = read_input(FILE_NAME);
  println!("Starting Puzzle 1");
  let start = Instant::now();
  let result = puzzle_1(input.clone());
  println!("Result: {}\n Calculated in {:?}", result, start.elapsed());

  println!("----------------------");
  println!("Starting Puzzle 2");
  let start = Instant::now();
  // let result = puzzle_2(template, rules, 40);
  println!("Result: {}\n Calculated in {:?}", 0, start.elapsed());
}


fn read_input(filename: &str) -> Vec<Vec<i8>>  {

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  // New way of reading in input all at once
  let input : Vec<Vec<i8>> = reader 
    .lines() // Read each line
    .map(|line| { //For each line, create a map that takes the line and converst it to a vec
      line.unwrap()
      .chars()
        .map(|c| c.to_digit(10).unwrap() as i8) // Convert each char in vec into an i8 
        .collect() // Collect all the i8s into an vec
    })
    .collect(); //Collect all the vecs into a vec

  input
}

fn puzzle_1(grid: Vec<Vec<i8>>) -> i8 {
  println!("Grid: {:?}", grid);
  0 
}