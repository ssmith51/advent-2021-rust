use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "test.txt";

struct HeightMap {
  readings: Vec<Vec<u32>>,
  max_x: usize,
  max_y: usize,
}

fn main() {
  println!("Advent of Code - Day 09");
  let height_map = read_input(FILE_NAME);
  println!("Max X: {}, Max Y: {}", height_map.max_x, height_map.max_y);
  
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
  height_map.max_x = height_map.readings[0].len();
  height_map.max_y = height_map.readings.len();

  height_map
}
