use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};
use std::collections::HashMap;

//Gobal file name for quick change
const FILE_NAME: &str = "test.txt";

fn main() {
  println!("Advent of Code - Day 12");
  read_input(FILE_NAME);
  println!("Starting Puzzle 1");
  let start = Instant::now();
  // let result = puzzle_1(template.clone(), rules.clone(), 10);
  println!("Result: {}\n Calculated in {:?}", 0, start.elapsed());

  println!("----------------------");
  println!("Starting Puzzle 2");
  let start = Instant::now();
  // let result = puzzle_2(template, rules, 40);
  println!("Result: {}\n Calculated in {:?}", 0, start.elapsed());
}


fn read_input(filename: &str)  {

  let mut grid: Vec<Vec<i8>> = Vec::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  let lines = reader.lines().map(|l| l.unwrap());
  grid.push(lines.map(|v| 
    v.chars().map(|c| c.to_digit(10)).collect()
  ).collect());

  println!("Grid: {:?}", grid);
 
}