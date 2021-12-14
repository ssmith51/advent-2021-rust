use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::VecDeque;

//Gobal file name for quick change
const FILE_NAME: &str = "small-test.txt";

// #[derive(Debug)]
// struct Cave {
//   name: String, 
//   is_small: bool, 
//   connected: Vec<String>, //Need to convert this to a Vec<Cave>
// }

fn main() {
  println!("Advent of Code - Day 12");
  let caves = read_input(FILE_NAME);
  println!("Caves Found: {:?}", caves);
  puzzle_1(caves.clone());
  puzzle_2(caves);
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

    // if existing_cave.is_none() { //If the cave does not exist, create a new one
    //   let is_small = name.chars().all(|c| c.is_ascii_lowercase()); //check if all the chars are lowercase
    //   let mut connected: Vec<String> = Vec::new();
    //   connected.push(parts[1].to_string());
  
    //     //Create a 'Cave'
    //   let c: Cave = Cave {
    //     name: name, 
    //     is_small: is_small,
    //     connected: connected,
    //   };
    //   caves.insert(parts[0].to_string(), c);
    // } else { // If the cave does exist, add a new cave
    //   existing_cave.unwrap().connected.push(parts[1].to_string());
    // }
  }

  caves
  
}


fn puzzle_1(caves: HashMap<String, Vec<String>> ) {
  //Find all paths from Start to End
  //Remove any paths that go through small twice 

  let mut paths: VecDeque<Vec<String>> = VecDeque::new();
  let mut valid_paths: Vec<Vec<String>> = Vec::new();
  let mut count = 0;
  
  paths.push_back(vec!["start".to_string()]);

  while let Some(path) = paths.pop_front() {
    println!("Path: {:?}", path);
    let p = path.last().unwrap();
    
    for next_path in caves.get(p).unwrap().iter() {

      let small = next_path.chars().all(|c| c.is_ascii_lowercase());

      if small && path.contains(next_path) {
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

  }
  
  

  println!("Valid Paths: {:?}", valid_paths);
  println!("Paths: {:?}", count);

}

fn puzzle_2(caves: HashMap<String, Vec<String>>) {
  
  let mut paths: VecDeque<Vec<String>> = VecDeque::new();
  let mut valid_paths: Vec<Vec<String>> = Vec::new();

  let mut count = 0;
  
  paths.push_back(vec!["start".to_string()]);

  while let Some(path) = paths.pop_front() {
    println!("Path: {:?}", path);

    let p = path.last().unwrap();
    
    for next_path in caves.get(p).unwrap().iter() {

      let mut small_visited = false;
      let small = next_path.chars().all(|c| c.is_ascii_lowercase());

      if next_path == "start" {
        continue;
      }

      if small && path.contains(next_path) {
        let mut c_count = 0; 

        for i in path.clone() {
          println!("I: {}", i);
          if next_path.clone() == i {
            c_count += 1;
          }

          let mut small_count = 0; 
          for j in path.clone() {
            if j == i {
              small_count += 1;
              
            }

            if small_count > 1 {
              small_visited = true;
            }

            if c_count > 1 && small_visited   {
              continue;
            }

          }
        }

        if c_count > 1  {
          continue;
        }

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

  }
  
  

  println!("Paths: {:?}", count);
  for mut vp in valid_paths {
    vp.push("end".to_string());
    println!("{:?}", vp);
  }


}
