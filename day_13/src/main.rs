use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Instant};

//Gobal file name for quick change
const FILE_NAME: &str = "test.txt";

fn main() {
    println!("Advent of Code - Day 12");
    let input = read_input(FILE_NAME);
    // println!("Caves Found: {:?}", caves);
    println!("Starting Puzzle 1");
    let start = Instant::now();
    // let count = puzzle_1(caves.clone());
    let duration = start.elapsed();
    println!("Total Duration {:?}", duration);
  
    // println!("----------------------");
    // println!("Starting Puzzle 2");
    // let start = Instant::now();
    // let count = puzzle_2(caves);
    // let duration = start.elapsed();
    // println!("Total Paths: {} calculated in: {:?}", count, duration);
}

fn read_input(filename: &str) {

}

// fn puzzle_1() {

// }
