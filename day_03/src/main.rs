use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

fn main() {
  println!("Advent of Code - Day 02");
  let report = read_input(FILE_NAME);

  println!("----------------------");
  println!("Starting Puzzle 1");
  let power = puzzle_1(&report);
  println!("Power Consumption: {}", power);
  
  println!("----------------------");
  println!("Starting Puzzle 2");
  let power = puzzle_2();
  println!("Power Consumption: {}", power.unwrap());

}

fn read_input(filename: &str) -> Vec<String>{
  println!("Reading file: {}", filename);

  let mut report: Vec<String> = Vec::new();
  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap(); //This is how to read in a line
    report.push(line);
  }

  report
}

fn puzzle_1(report: &Vec<String>) -> i64 {

  let (mut pos, mut one_count, mut zero_count, mut gamma, mut eplison): (usize, i64, i64, String, String) = (0, 0, 0,"".to_string(),"".to_string());

  while pos < report[0].len() {

    for line in report {

      let c = line.chars().nth(pos).unwrap();
      if c == '1' {
        zero_count = zero_count + 1;
      } else if c == '0' {
        one_count = one_count + 1;
      }
    }

    if zero_count > one_count {
      gamma.push('0');
      eplison.push('1');
    } else {
      gamma.push('1');
      eplison.push('0');
    }

    zero_count = 0;
    one_count = 0;
    pos = pos + 1;
  }

  println!("Gamma Binary: {}", gamma);
  println!("Eplison Binary: {}", eplison);

  //Convert gamma & eplison from String to str and parse to i64s
  let gamma: i64 = i64::from_str_radix(&gamma[..], 2).unwrap();
  let eplison: i64 = i64::from_str_radix(&eplison[..], 2).unwrap();

  let power: i64 =  gamma * eplison;
  power
 
}

//WTF is going on here :) 
fn puzzle_2() -> Result<u32, Box<dyn std::error::Error>> {

  //Read in the test file from the root dir as a string
  let report = include_str!("../test.txt");

  //I still need to understand how this works better... took it from https://github.com/wfxr/advent-of-code-2021/blob/main/src/day03/mod.rs 
  //to learn different way sto program in Rust. 
  let rating = |most_common: bool| -> Result<u32, Box<dyn std::error::Error>> {
    let mut seq: Vec<_> = report.lines().collect();
    let mut col = 0;
    while seq.len() > 1 {
        let ones = seq.iter().filter(|l| l.as_bytes()[col] == b'1').count();
        let bit = match (most_common, ones * 2 >= seq.len()) {
            (true, true) | (false, false) => b'1',
            _ => b'0',
        };
        seq = seq.into_iter().filter(|l| l.as_bytes()[col ] == bit).collect();
        col += 1;
    }
    Ok(u32::from_str_radix(seq.first().ok_or("empty input")?, 2)?)
  };
  let (oxygen, co2) = (rating(true)?, rating(false)?);
  Ok(oxygen * co2)
}
