use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};
use std::collections::HashMap;

//Gobal file name for quick change
const FILE_NAME: &str = "test.txt";

fn main() {
  println!("Advent of Code - Day 12");
  let input = read_input(FILE_NAME);
  println!("Input: {:?}", input);
  println!("Starting Puzzle 1");
  let start = Instant::now();
  let result = puzzle_1(input.clone());
  println!("Result: {}\n Calculated in {:?}", result, start.elapsed());

  println!("----------------------");
  println!("Starting Puzzle 2");
  let start = Instant::now();
  // let result = puzzle_2(input);
  println!("Result: {}\n Calculated in {:?}", 0, start.elapsed());
}

fn read_input(filename: &str) -> String  {

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  // New way of reading in input all at once

  
  let input: Vec<String> = reader 
    .lines() // Read each line
    .map(|line| { //For each line, create a map that takes the line and converst it to a vec
      line.expect("Could not parse input")
    })
    .collect();

  input[0].to_string()
}

fn puzzle_1(hex_input: String) -> i8 {

  let binary_map = get_binary_map();
  let mut binary: Vec<char> = Vec::new();


  //Convert Hex Input to a binary value
  for c in hex_input.chars() {
    for v in  binary_map.get(&c).unwrap(){
      binary.push(*v);
    }
  }

  println!("Binary Vec: {:?}", binary);

  let mut header: Vec<char> = binary[0..3].to_vec();
  header.insert(0, '0'); // Pad header with extra 0

  let mut id: Vec<char> = binary[3..6].to_vec();
  id.insert(0, '0'); // Pad ID with extra 0

  println!("Header: {:?}, ID: {:?}", header, id);


  //Temp - Return 1
  0

}

fn get_binary_map() -> HashMap<char, Vec<char>> {

  let mut binary_map: HashMap<char, Vec<char>> = HashMap::new();
  binary_map.insert( '0', "0000".chars().collect());
  binary_map.insert( '1', "0001".chars().collect());
  binary_map.insert( '2', "0010".chars().collect());
  binary_map.insert( '3', "0011".chars().collect());
  binary_map.insert( '4', "0100".chars().collect());
  binary_map.insert( '5', "0101".chars().collect());
  binary_map.insert( '6', "0110".chars().collect());
  binary_map.insert( '7', "0111".chars().collect());
  binary_map.insert( '8', "1000".chars().collect());
  binary_map.insert( '9', "1001".chars().collect());
  binary_map.insert( 'A', "1010".chars().collect());
  binary_map.insert( 'B', "1011".chars().collect());
  binary_map.insert( 'C', "1100".chars().collect());
  binary_map.insert( 'D', "1101".chars().collect());
  binary_map.insert( 'E', "1110".chars().collect());
  binary_map.insert( 'F', "1111".chars().collect());

  binary_map

}