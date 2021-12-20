use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};
use std::cmp::{min};

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

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


fn read_input(filename: &str) -> Vec<Vec<i64>>  {

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  // New way of reading in input all at once
  let input : Vec<Vec<i64>> = reader 
    .lines() // Read each line
    .map(|line| { //For each line, create a map that takes the line and converst it to a vec
      line.unwrap()
      .chars()
        .map(|c| c.to_digit(10).unwrap() as i64) // Convert each char in vec into an i64 
        .collect() // Collect all the i64s into an vec
    })
    .collect(); //Collect all the vecs into a vec

  input
}

fn puzzle_1(grid: Vec<Vec<i64>>) -> i64 {
  // println!("Grid: {:?}", grid);
  let max_y = grid.len() as i64;
  let max_x = grid[0].len() as i64;

  println!("Calculating Cost");
  let total = calc_cost(&grid);

  total
}

fn calc_cost(grid: &Vec<Vec<i64>>) -> i64 {
//fn calc_cost(grid: &Vec<Vec<i64>>, x: i64, y: i64) -> i64 {
  let mut cost: i64 = 0;

  let max_y = grid.len() as usize;
  let max_x = grid[0].len() as usize;

  let mut cache: Vec<Vec<i64>> = vec![vec![0; max_x]; max_y];

  for i  in 0..max_y {

  }


  //https://www.techiedelight.com/find-minimum-cost-reach-last-cell-matrix-first-cell/
  
  

  // print!(".");

  // if x == 0 || y == 0 {
  //   cost = i64::MAX;
  // } else if x == 1 && y == 1 {
  //   cost = 0;
  // } else {
  //   cost = min(calc_cost(grid, x, y-1), calc_cost(grid, x-1, y)) + grid[(y-1) as usize] [(x-1) as usize];
  // }

  cost

}
