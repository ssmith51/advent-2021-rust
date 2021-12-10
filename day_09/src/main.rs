use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

struct HeightMap {
  readings: Vec<Vec<u32>>,
  max_x: usize,
  max_y: usize,
}

fn main() {
  println!("Advent of Code - Day 09");
  let height_map = read_input(FILE_NAME);
  println!("----------------------");
  println!("Starting Puzzle 1");
  let result = puzzle_1(&height_map);
  println!("Sum of Risk Levels: {}", result);
  println!("----------------------");
  println!("Starting Puzzle 2");
  let result = puzzle_2(&height_map);
  println!("Three Largest Basins: {}", result);
  
}

fn read_input(filename: &str) -> HeightMap {
  
  //Init the height map
  let mut height_map: HeightMap = HeightMap{
    readings: Vec::new(),
    max_x: 0,
    max_y: 0,
  };

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    const RADIX: u32 = 10; 

    //Convert to line to a vec of numbers
    let r = line.unwrap()
      .chars()
      .map(|c| c.to_digit(RADIX).unwrap())
      .collect();

    //Add the 'row' to the overall map
    height_map.readings.push(r);
  }

  //Set the X & Y values for the grid
  height_map.max_x = height_map.readings[0].len() -1;
  height_map.max_y = height_map.readings.len() -1 ;

  height_map
}

fn puzzle_1(height_map: &HeightMap) -> i32 {
  let mut sum: i32 = 0;

  let (mut y, mut x): (usize, usize) = (0,0);

  for row in &height_map.readings {

    for val in row {
      
      //Calculate comparision indexes
      let bottom :i32 = if y == height_map.max_y {10} else {(height_map.readings[(y+1)][x])as i32};
      let top: i32 = if y == 0 {10} else {height_map.readings[(y-1)][x] as i32};
      let prev: i32 = if x == 0 {10} else {row[(x-1)] as i32};
      let next :i32 = if x < height_map.max_x {row[(x+1)] as i32} else {10};

      let val = *val as i32;

      if val < prev && val < next && val < top && val < bottom {
        sum += val+1;
      }
      x += 1;
    } 

    //Rest X & Increment Y
    x = 0; 
    y += 1;
  }

  sum 
}

fn puzzle_2(height_map: &HeightMap) -> i32 {
  let mut sum: i32 = 0;

  let (mut y, mut x): (usize, usize) = (0,0);

  for row in &height_map.readings {

    for val in row {
      
      //Calculate comparision indexes
      let bottom :i32 = if y == height_map.max_y {10} else {(height_map.readings[(y+1)][x])as i32};
      let top: i32 = if y == 0 {10} else {height_map.readings[(y-1)][x] as i32};
      let prev: i32 = if x == 0 {10} else {row[(x-1)] as i32};
      let next :i32 = if x < height_map.max_x {row[(x+1)] as i32} else {10};

      let val = *val as i32;

      if val < prev && val < next && val < top && val < bottom {

        //Need to do some type of basic graph searching here, keeping track of the total size
        //Maybe use a recurisve function to constantly shift the grids and track the basin...

        sum += val+1;
      }
      x += 1;
    } 

    //Rest X & Increment Y
    x = 0; 
    y += 1;
  }

  sum 
}
