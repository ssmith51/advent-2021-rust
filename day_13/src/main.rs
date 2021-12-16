use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

type Point = (usize, usize);

#[derive(Debug, Clone)]
struct Grid {
    readings: Vec<Vec<usize>>,
    folds: Vec<(String, usize)>,
  }

fn main() {
    println!("Advent of Code - Day 12");
    let input = read_input(FILE_NAME);
    println!("Starting Puzzle 1");
    let start = Instant::now();
    let result = puzzle_1(input.clone(), false);
    let duration = start.elapsed();
    println!("Total Dots: {}. Calculated In {:?}", result, duration);
  
    // println!("----------------------");
    // println!("Starting Puzzle 2");
    // let start = Instant::now();
    // let count = puzzle_2(caves);
    // let duration = start.elapsed();
    // println!("Total Paths: {} calculated in: {:?}", count, duration);
}

fn read_input(filename: &str) -> Grid{

    let mut grid: Grid = Grid{
        readings: Vec::new(),
        folds: Vec::new(),
    };

    let fi = File::open(filename).unwrap();
    let reader = BufReader::new(fi);

    let mut points: Vec<Point> = Vec::new();
    let (mut max_x, mut max_y) = (0,0);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line.contains("fold") {

            let parts: Vec<&str> = line.split(" ")
                .collect::<Vec<&str>>()[2]
                .split("=")
                .collect();
                
            let f = (parts[0].to_string(), parts[1].parse().unwrap() );
            grid.folds.push(f);

        } else if line.len() > 0 {
            let parts: Vec<usize> = line.split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            
            let point: Point = (parts[0], parts[1]);

            //Find the Max X and Y Values
            max_x = if point.0 > max_x {point.0} else {max_x};
            max_y = if point.1 > max_y {point.1} else {max_y};
            points.push(point);

        }
    }
    max_x += 1;
    max_y += 1;
    grid.readings = vec![vec![0; max_x]; max_y];
    for point in points {
        grid.readings[point.1][point.0] = 1;
    }

    grid

}

fn puzzle_1(mut grid: Grid, debug: bool) -> i32 {
    let mut count = 0; 

    if debug {
        for row in &grid.readings {
            println!("{:?}", row);
        }
    }

    let first_fold = &grid.folds.get(0).unwrap();
    println!("First Fold: {:?}", first_fold);

    if first_fold.0 == "y" {
      grid = fold_y(grid.clone(), first_fold.1);
    } else if first_fold.0 == "x" {
      grid = fold_x(grid.clone(), first_fold.1);
    }

    if debug {
      for row in &grid.readings {
          println!("{:?}", row);
      }
    }
    
    for row in grid.readings {
      for val in row {
        if val > 0 {
          count += 1;
        }
      }
    }
    
    // count += 1; 

    

    count

}

// fn fold_x() {

// }

fn fold_y(mut grid: Grid, fold: usize) -> Grid {

  let mut new_grid: Vec<Vec<usize>> = vec![vec![0; grid.readings[0].len()]; fold];
  // println!("New Grid: {:?}", new_grid);

  let max_x = grid.readings.get(0).unwrap().len();
  let max_y = grid.readings.len();

  //Transpose Top of the Fold
  for y in 0..fold {
    for x in 0..max_x {
      if grid.readings[y][x] > 0 {
        new_grid[y][x] = 1;
      }
    }
  }

  //Transpose the 'Flip' of the fold
  let mut y: usize = max_y -1;
  let mut new_y: usize = 0;
  while y > fold {
    for x in 0..max_x{
      if grid.readings[y][x] > 0 {
        new_grid[new_y][x] = 1;
      }
    }

    y -= 1;
    new_y += 1;
  }
  
  // println!("New Grid: {:?}", new_grid);

  grid.readings = new_grid;
  grid
}

fn fold_x(mut grid: Grid, fold: usize) -> Grid {

  let mut new_grid: Vec<Vec<usize>> = vec![vec![0; fold]; grid.readings.len()];

  let max_x = grid.readings.get(0).unwrap().len();
  let max_y = grid.readings.len();

  for y in 0..max_y {
    for x in 0..fold {
      if grid.readings[y][x] > 0 {
        new_grid[y][x] = 1;
      }
    }
  }

  grid.readings = new_grid;
  grid
}
