use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};
use std::cmp::{min};

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
  let result = puzzle_2(input);
  println!("Result: {}\n Calculated in {:?}", result, start.elapsed());
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
  println!("Calculating Cost");
  let total = calc_cost(&grid);
  total
}

fn puzzle_2(grid: Vec<Vec<i64>>) -> i64 {
  println!("Calculating Cost");

  //Rebuild Grid to be 5x larger
  let max_y = grid.len() as usize;
  let max_x = grid[0].len() as usize;
  
  //New Empty Grid
  let mut new_grid: Vec<Vec<i64>> = vec![vec![0; max_x *5 ]; max_y * 5];
  //Send new grid to calculate cost
  println!("Starting Grid X: {}, Starting Grid Y: {}", max_x, max_y);
  println!("New Grid X: {}, New Grid Y: {}", new_grid[0].len(), new_grid.len());

  for y in 0..max_y {
    // let mut new_row = String::new();
    for x in 0..max_x {

      let mut val = grid[y][x]; 
      new_grid[y][x] = val;

      for n in 1..5 {
        val = new_val(val);
        new_grid[y][x + (max_x * n)] = val;
        new_grid[y + (max_y * n)][x] = val;
        // new_grid[y][x + (max_x * n)] = new_val(val);
        // new_grid[y + (max_y * n)][x + (max_x * n)] = val;
      }


      // let mut val = grid[y][x]; 
      // new_grid[y][x] = val;

      // val = new_val(val);
      // new_grid[y][x + max_x] = val;
      // new_grid[y + max_y][x] = val; 

      // val = new_val(val);
      // new_grid[y][x + (max_x * 2)] = val;
      // new_grid[y + (max_y *2)][x + (max_x * 2)] = val;

      // val = new_val(val);
      // new_grid[y][x+ (max_x * 3)]= val;
      // new_grid[y + (max_y *3)][x+ (max_x * 3)] = val;

      // val = new_val(val);
      // new_grid[y][x+ (max_x * 4)] = val;
      // new_grid[y + (max_y *4)][x+ (max_x * 4)] = val;

      // new_row.push(val.to_string().chars().nth(0).unwrap());


    }
    // println!("{}", new_row);
  }

  //Send new grid to calculate cost
  println!("Starting Grid X: {}, Starting Grid Y: {}", max_x, max_y);
  
  println!("New Grid X: {}, New Grid Y: {}, Value: {}", new_grid[0].len(), new_grid.len(), new_grid[49][49]);

  for y in 0..new_grid.len() {
    // let mut new_row = String::new();
    for x in 0..new_grid[0].len() {
      print!("{}", new_grid[y][x]);
    }
    println!("");
  }

  let total = calc_cost(&new_grid);
  total
}

fn new_val(val: i64) -> i64 {
  let mut val = val;
  if val == 9 {
    val = 1; 
  } else {
    val += 1;
  }
  val

}

fn calc_cost(grid: &Vec<Vec<i64>>) -> i64 {

  let max_y = grid.len() as usize;
  let max_x = grid[0].len() as usize;

  let mut cache: Vec<Vec<i64>> = vec![vec![0; max_x]; max_y];

  for i  in 0..max_y {

    for j in 0..max_x {

      cache[i][j] = grid[i][j];

      if i == 0 && j > 0 {
        cache[0][j] += cache[0][j - 1];
      } 
      // fill the first column (there is only one way to reach any cell
      // in the first column from its adjacent top cell)
      else if j == 0 && i > 0 {
        cache[i][0] += cache[i - 1][0];
      }

      // fill the rest with the matrix (there are two ways to reach any cell
      // in the rest of the matrix, from its adjacent left
      // cell or adjacent top cell)
      else if i > 0 && j > 0 {
        cache[i][j] += min(cache[i - 1][j], cache[i][j - 1]);
      }


    }

  }
  

  let cost = cache[max_y - 1][max_x - 1] - cache[0][0];
  cost

}
