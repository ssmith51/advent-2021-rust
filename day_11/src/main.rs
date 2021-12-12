use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

fn main() {
  println!("Advent of Code - Day 11");
  let input = read_input(FILE_NAME);
  println!("Starting Puzzle 1");
  let result = puzzle_1(input);
  println!("Total Flashes: {}", result);

  println!("----------------------");
  let input = read_input(FILE_NAME); //Reload the input
  println!("Starting Puzzle 2");
  let result = puzzle_2(input);
  println!("Total Step: {}", result);
  
}


fn read_input(filename: &str) -> Vec<Vec<i8>> {

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

fn puzzle_1(mut grid: Vec<Vec<i8>>) -> i32 {
  println!("Starting Grid: {:?}", grid);

  let mut flash_count: i32 = 0;

  //Iterate from 0 -> 99 ignoring the index. To make this 0 -> 100 the syntax is 0..=100
  //Create a Vec of coordinates that repersent each flash
  let mut flashes: Vec<(usize, usize)> = Vec::new();
  for _ in 0..100 {

    // Iterate over every item and add engry to the values. Note - the grid is mutable based on the function input
    for (y, row) in grid.iter_mut().enumerate() {
      for (x, col) in row.iter_mut().enumerate() {

        //Increase the flashes
        *col += 1;

        //Create a Vec of flashes to 'flash'
        if *col > 9 {
          flashes.push((y,x)); //Add to the end of the queue
        }

      }
    }

    //Iterate over the points that have flashed
    while flashes.len() > 0 {
      let (y,x) = flashes.pop().unwrap();

      //Rest coordinate to 0 & increment flash count
      grid[y][x] = 0;
      flash_count += 1;
    

      for (ny, nx) in get_neighbors(y, x) {

        if grid[ny][nx] == 0 || grid[ny][nx] > 9 { //Has the neighbor flashed already? 
          continue;
        }

        grid[ny][nx] += 1;

        if grid[ny][nx] > 9 {
          flashes.push((ny, nx));
        }

      }

      
    }
  }
  println!("Final Grid: {:?}", grid);
  flash_count
}

fn puzzle_2(mut grid: Vec<Vec<i8>>) -> i32 {

  //Iterate from 0 -> 99 ignoring the index. To make this 0 -> 100 the syntax is 0..=100
  //Create a Vec of coordinates that repersent each flash
  let mut flashes: Vec<(usize, usize)> = Vec::new();
  let mut step = 0;
  loop{
    step += 1;
    // Iterate over every item and add engry to the values. Note - the grid is mutable based on the function input
    for (y, row) in grid.iter_mut().enumerate() {
      for (x, col) in row.iter_mut().enumerate() {

        //Increase the flashes
        *col += 1;

        //Create a Vec of flashes to 'flash'
        if *col > 9 {
          flashes.push((y,x)); //Add to the end of the queue
        }

      }
    }
    let mut flash_count = 0;
    //Iterate over the points that have flashed
    while flashes.len() > 0 {
      let (y,x) = flashes.pop().unwrap();

      //Rest coordinate to 0 & increment flash count
      grid[y][x] = 0;
      flash_count += 1;
    
      
      for (ny, nx) in get_neighbors(y, x) {

        if grid[ny][nx] == 0 || grid[ny][nx] > 9 { //Has the neighbor flashed already? 
          continue;
        }

        grid[ny][nx] += 1;

        if grid[ny][nx] > 9 {
          flashes.push((ny, nx));
        }

      }
    }

    if flash_count == 100 {
      break;
    }

  }
  println!("Final Grid: {:?}", grid);

  step
}


//This function is used to get all the neighbords of any given point in the grid
fn get_neighbors(y: usize, x: usize) -> Vec<(usize, usize)>  {

  let mut neighbors = Vec::new();

  for i in -1..=1 {

    let cur_i = (y as i32) + i; //Check to see if the neighbor is outside the bounds of the grid on the y axis
    if cur_i < 0 || 10 <= cur_i {
      continue;
    }

    for j in -1..=1 {

      if i == 0 && j == 0 { //Skip over original number since 0,0 maps to it
        continue;
      }

      let cur_j = (x as i32) + j;  
      if cur_j < 0  || 10 <= cur_j { //Check to see if the neighbor is outside the bounds of the grid on the x axis
        continue;
      }

     

      neighbors.push((cur_i as usize, cur_j as usize)); 
    }
  }

  neighbors
}