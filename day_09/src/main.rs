use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "test.txt";
const NEXT: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];


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
  good_answer();
  
  
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
      if is_low_point(x, y, &height_map, row, *val) {
        sum += (val +1) as i32;
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
  

  let (mut y, mut x): (usize, usize) = (0,0);

  let mut basins: Vec<i32> = Vec::new();
  let found: Vec<(usize, usize)> = Vec::new();

  for row in &height_map.readings {

    for val in row {
      let mut basin_size: i32 = 1;
      //Calculate comparision indexes
      if is_low_point(x, y, &height_map, row, *val) {
        println!("-- Basin: {} started", val);
        let (result, _found) =calc_basin(*val, x, y, height_map, &found);
        basin_size += result;
        basins.push(basin_size);
        println!("-- Basin: {} complete", val);
      }


      x += 1;
    } 

    //Rest X & Increment Y
    x = 0; 
    y += 1;
  }

  println!("Basins: {:?}", basins);

  basins.len() as i32 
}

fn is_low_point(x: usize, y: usize, height_map: &HeightMap, row: &Vec<u32>, val: u32) -> bool{

    let mut low_point: bool = false; 

    //Calculate comparision indexes
    let bottom :i32 = if y == height_map.max_y {10} else {(height_map.readings[(y+1)][x])as i32};
    let top: i32 = if y == 0 {10} else {height_map.readings[(y-1)][x] as i32};
    let prev: i32 = if x == 0 {10} else {row[(x-1)] as i32};
    let next :i32 = if x < height_map.max_x {row[(x+1)] as i32} else {10};

    let val = val as i32;

    if val < prev && val < next && val < top && val < bottom {

    //   //Need to do some type of basic graph searching here, keeping track of the total size
    //   //Maybe use a recurisve function to constantly shift the grids and track the basin...
    //   let mut basin_size: i32 = 0;
    //   basin_size = if top < 10 {basin_size +1 } else {basin_size};
    //   basin_size = if bottom < 10 {basin_size +1 } else {basin_size};
    //   basin_size = if prev < 10 {basin_size +1 } else {basin_size};
    //   basin_size = if next < 10 {basin_size +1 } else {basin_size};

    //   println!("Basin Size: {}", basin_size);

      low_point = true;
    }

    low_point
     

}

fn calc_basin(val: u32, x: usize, y: usize, height_map: &HeightMap, found: &Vec<(usize, usize)>) -> (i32, Vec<(usize,usize)>) {
  println!("!Checking: {} at position Y: {}, X: {}", val, y, x);
  let mut found = found.clone();
  let mut basin_size: i32 = 0;
  let val = val;
  let x = x; 
  let y = y;

  //Calculate comparision indexes
  let bottom :u32 = if y == height_map.max_y {10} else {(height_map.readings[(y+1)][x])as u32};
  let top: u32 = if y == 0 {10} else {height_map.readings[(y-1)][x] as u32};
  let prev: u32 = if x == 0 {10} else {height_map.readings[y][(x-1)] as u32};
  let next: u32 = if x < height_map.max_x {height_map.readings[y][(x+1)] as u32} else {10};

  println!(" Val: {}, Bottom: {}, Top: {}, Prev: {}, Next: {}", val, bottom, top, prev, next);
  let previous = found.iter().find(|&&n| n == (y,x));
  println!("Pervious Basin {:?}", previous);
  println!("Pervious Found {:?}", found);


  let bottom_match = val < bottom && bottom < 9 ; 
  let top_match = val < top && top < 9 && previous.is_none();
  let prev_match = val < prev && prev < 9 && previous.is_none(); 
  let next_match = val < next && next < 9 && previous.is_none();

  println!(" BM: {}, TM: {}, PM: {}, NM: {}", bottom_match, top_match, prev_match, next_match);

  
  if bottom_match  {
    // println!(" Bottom Match! Val: {}, Bottom: {}", val, bottom);
    basin_size += 1;
    let (result, f) = calc_basin(bottom, x, y+1, height_map, &found);
    found = f;
    found.push((y,x));
    basin_size += result;
  } 
  
  if top_match {
    // println!(" Top Match! Val: {}, Top: {}", val, top);
    basin_size += 1;
    let (result, f) = calc_basin(top, x, y-1, height_map, &found);
    found = f;
      found.push((y,x));
    basin_size += result;
  } 
  
  if prev_match {
    // println!(" Prev Match! Val: {}, Prev: {}", val, prev);
    basin_size += 1; 
    let (result, f) = calc_basin(prev, x-1, y, height_map, &found);
    found = f;
      found.push((y,x));
    basin_size += result;
  } 
  
  if next_match {
    // println!(" Next Match! Val: {}, Next {}", val, next);
    basin_size +=1; 
    let (result, f) = calc_basin(next, x+1, y, height_map, &found);
    found = f;
      found.push((y,x));
    basin_size += result;
  }
  
  
  println!("!Done Checking: {} at position X: {}, Y: {}", val, x, y);
  (basin_size, found)

}

fn good_answer() {
  let mut input = include_bytes!("../input.txt")
  .split(|&b| b == b'\n')
  .map(|l| l.to_vec())
  .collect::<Vec<Vec<u8>>>();

  let mut basins = vec![];
  for y in 0..input.len() {
    for x in 0..input[0].len() {
        (input[y][x] < b'9').then(|| basins.push(basin(&mut input, x, y)));
    }
  }

  basins.sort_unstable();
  println!("{}", basins.iter().rev().take(3).product::<usize>());
  }

  fn basin(map: &mut Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    map[y][x] = b'9';
    NEXT.iter()
        .map(|(xx, yy)| ((x as isize + xx) as usize, (y as isize + yy) as usize))
        .fold(1, |acc, (x, y)| {
            match map.get(y).and_then(|l| l.get(x)).map(|&n| n < b'9') {
                Some(true) => acc + basin(map, x, y),
                _ => acc,
            }
        })
}
