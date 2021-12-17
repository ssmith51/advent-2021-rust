use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};
use std::collections::HashMap;

//Gobal file name for quick change
const FILE_NAME: &str = "input.txt";

fn main() {
  println!("Advent of Code - Day 12");
  let (template, rules) = read_input(FILE_NAME);
  println!("Starting Puzzle 1");
  let start = Instant::now();
  let result = puzzle_1(template.clone(), rules.clone(), 10);
  let duration = start.elapsed();
  println!("Result: {}\n Calculated in {:?}", result, duration);

  println!("----------------------");
  println!("Starting Puzzle 2");
  let start = Instant::now();
  let result = puzzle_1(template, rules, 40);
  let duration = start.elapsed();
  println!("Result: {}\n Calculated in {:?}", result, duration);


}



fn read_input(filename: &str) -> (Vec<char>,  HashMap<(char,char), char>) {
  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  let mut rules: HashMap<(char,char), char> = HashMap::new();
  let mut template: Vec<char> = Vec::new();


  for (i, line) in reader.lines().enumerate() {

    let line = line.unwrap();

    if i == 0 {
      template = line.chars().collect();

    } else if i > 1 {

      let key_1 = line.chars().nth(0).unwrap();
      let key_2 = line.chars().nth(1).unwrap();
      let result = line.chars().nth(6).unwrap();
      rules.insert((key_1, key_2),result);
    }

  }

  (template, rules)
}

fn puzzle_1(template: Vec<char>, rules: HashMap<(char, char), char>, steps: usize) -> i32 {
  

  let mut template = template.clone();
  println!("Template: {:?}", template);
  println!("Rules: {:?}", rules);

  for i in 0..steps {
    let start = Instant::now();
    println!("Iteration: {}", i);
    let mut polymer: Vec<char> = vec!['.'; 2 *template.len() - 1]; // Always add n-1 to the polymer
    for x in 0..template.len() -1 {
      polymer[2 * x] = template[x];
      polymer[2 * x + 1] = *rules.get(&(template[x], template[x+1])).unwrap();
      polymer[2 * x + 2] = template[x + 1]
    }
    template = polymer;
    println!("Iteration: {} Completed in: {:?}", i, start.elapsed());
  }

  let mut letters: HashMap<char, i32> = HashMap::new();

  for c in template {
    letters.entry(c) 
      .and_modify(|v| {*v += 1}) 
      .or_insert(1);
  }

  let mut most_common: (char, i32) = ('.', 0);
  let mut least_common: (char, i32) = ('.', 1000000000); //Some Arbitary large number.... bad job here :) 

  for l in letters.clone() {

    if l.1 > most_common.1 {
      most_common = (l.0, l.1);
    } 

    if l.1 < least_common.1 {
      least_common = (l.0, l.1);
    }

  }

  println!("{:?}", letters);
  println!("Least Common: {:?}, Most Common: {:?}", least_common, most_common);
  
  let result = most_common.1 - least_common.1; 
  result

}