use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::{Instant};

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

// #[derive(Debug)]
// struct Cave {
//   name: String, 
//   is_small: bool, 
//   connected: Vec<String>, //Need to convert this to a Vec<Cave>
// }

fn main() {
  println!("Advent of Code - Day 12");
  let caves = read_input(FILE_NAME);
  // println!("Caves Found: {:?}", caves);
  println!("Starting Puzzle 1");
  let start = Instant::now();
  let count = puzzle_1(caves.clone());
  let duration = start.elapsed();
  println!("Total Paths: {} calculated in: {:?}", count, duration);

  println!("----------------------");
  println!("Starting Puzzle 2");
  let start = Instant::now();
  let count = puzzle_2(caves);
  let duration = start.elapsed();
  println!("Total Paths: {} calculated in: {:?}", count, duration);

}

fn read_input(filename: &str) -> HashMap<String, Vec<String>>{

  let mut caves: HashMap<String, Vec<String>> = HashMap::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();

    let parts: Vec<&str> = line.split("-").collect();
    let cave_1 = parts[0].to_string();
    let cave_2 = parts[1].to_string();
    
    //Try building a hashmap of caves and their connected caves
    caves.entry(cave_1.clone()) 
      .and_modify(|neighbor| neighbor.push(cave_2.clone()))
      .or_insert(vec![cave_2.clone()]);

    caves.entry(cave_2)
      .and_modify(|neighbor| neighbor.push(cave_1.clone()))
      .or_insert(vec![cave_1]);

  }

  caves
  
}


fn puzzle_1(caves: HashMap<String, Vec<String>> ) -> i64 {

  let mut paths: VecDeque<Vec<String>> = VecDeque::new();
  let mut count = 0;
  
  paths.push_back(vec!["start".to_string()]);

  while let Some(path) = paths.pop_front() {
    let p = path.last().unwrap();
    
    for next_path in caves.get(p).unwrap().iter() {

      let small = next_path.chars().all(|c| c.is_ascii_lowercase());

      if small && path.contains(next_path) {
        continue;
      }

      if next_path == "end" {
        count +=1;
        let mut v = path.clone();
        v.push("end".to_string());
        continue;
      }

      let mut new_path = path.clone();
      new_path.push(next_path.clone());
      paths.push_back(new_path);


    }

  }
  
  count

}

fn puzzle_2(caves: HashMap<String, Vec<String>>) -> i64 {
  
  let mut paths: VecDeque<Vec<String>> = VecDeque::new();
  let mut valid_paths: Vec<Vec<String>> = Vec::new();
  let mut count = 0;
  
  paths.push_back(vec!["start".to_string()]);

  while let Some(path) = paths.pop_front() {
    // println!("Path: {:?}", path);
    let p = path.last().unwrap();
    
    for next_path in caves.get(p).unwrap().iter() {

      if next_path == "start" {
        continue;
      }

      let small = next_path.chars().all(|c| c.is_ascii_lowercase());

      let mut small_visited = false; 
      if small  {
        //Check to see if the cave exists anywhere else in the path
        if path.contains(next_path) {
          //Check to see if any other small cave is duplicated in the path

          for i in path.clone() {

            if  i.chars().all(|c| c.is_ascii_lowercase()) && (i != "start" || i != "end"){
              let count = path.iter().filter(|&n| *n == i).count();
              // println!("Count of {}: {}", i, count);
              if count > 1 {
                small_visited = true; 
              }
              
            }

          }

        }
      
      }

      if small_visited {
        continue;
      }


      if next_path == "end" {
        count +=1;
        let v = path.clone();
        valid_paths.push(v);
        continue;
      }

      let mut new_path = path.clone();
      new_path.push(next_path.clone());
      paths.push_back(new_path);
    }

    // println!("Path: Complete");
    

    
  }


  count
}
