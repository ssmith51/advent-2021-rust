use std::fs::File;
use std::io::{BufRead, BufReader};


//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

fn main() {
    println!("Advent of Code - Day 01");
    let depths = read_input(FILE_NAME);

    //I only want to validate test input!
    if FILE_NAME.eq("test.txt") {
        println!("Depths Read In: {:?}", depths);
    }

    println!("----------------------");
    println!("Starting Puzzle 1");
    let puzzle1_result = puzzle_1(&depths);
    println!("Measurement: {}", puzzle1_result);
    println!("----------------------");

    println!("----------------------");
    println!("Starting Puzzle 2");
    let puzzle2_result = puzzle_2(&depths);
    println!("Measurement: {}", puzzle2_result);
    println!("----------------------");

}

fn read_input(filename: &str) -> Vec<i64> {
    println!("Reading file: {}", filename);
    let fi = File::open(filename).unwrap();
    let reader = BufReader::new(fi);

    let mut depths: Vec<i64> = Vec::new(); //Vec must be mutable to push to it

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); //This is how to read in a line
        let num: i64 = line.parse().unwrap(); //This is how to convert a String to an i64
        depths.push(num)
    }

    depths

}

fn puzzle_1(depths: &Vec<i64>) -> i64 {

    //Copy the depths to a new Vector
    let depths = depths.to_vec();

    let mut previous_depth: i64 = -1;
    let mut count: i64 = 0;

    for depth in depths {

        if depth > previous_depth && previous_depth != -1{
            count = count+1;
        }
        previous_depth = depth
    }

    count
} 

fn puzzle_2(depths: &Vec<i64>) -> i64 {
    let (mut i, mut count): (usize, i64) = (0,0);

    //Calculate the total number of depths to read (depth length minus 3)
    let len_depths: usize = depths.len() - 4; //Sub 4 to account of indexing below

    while i <= len_depths {
        let a = depths[i] + depths[i+1] + depths[i+2];
        let b = depths[i+1] + depths[i+2] + depths[i+3];

        if b > a {
            count = count + 1
        }
        
        i = i + 1;
    }

    count
}