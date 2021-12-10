use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

fn main() {
    println!("Advent of Code - Day 10");
    let input = read_input(FILE_NAME);
    
    println!("----------------------");
    println!("Starting Puzzle 1");
    let result = puzzle_1(&input);
    println!("Final Score: {}", result);

    println!("----------------------");
    println!("Starting Puzzle 2");
    let result = puzzle_2(&input);
    println!("Final Score: {}", result);

}

fn read_input(filename: &str) -> Vec<String> {
  
  let mut input: Vec<String> = Vec::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    input.push(line.unwrap());
  }

  input
}

fn puzzle_1(input: &Vec<String>) -> i64 {
  let mut score: i64 = 0;

  for line in input {

    let mut expected: Vec<char> = Vec::new();
    let mut illegal: char = '|';
    for (_index, c) in  line.chars().enumerate() {

      if c == '(' {
        expected.push(')');
      } else if c == '[' {
        expected.push(']');

      } else if c == '{' {
        expected.push('}');

      } else if c == '<' {
       expected.push('>');

      } else if &c != expected.get(expected.len() - 1).unwrap() {
        illegal = c;
        break;
      } else if &c == expected.get(expected.len() - 1).unwrap() {
        expected.pop();
      }

    }

    //Calculate Illegal Score
    if illegal == ')' {
      score += 3; 
    } else if illegal == ']' {
      score += 57;
    } else if illegal == '}' {
      score += 1197;
    } else if illegal == '>' {
      score += 25137;
    }

  }


  score

}


fn puzzle_2(input: &Vec<String>) -> i64 {
  let mut score: i64 = 0;
  let mut scores: Vec<i64> = Vec::new();

  for line in input {

    let mut expected: Vec<char> = Vec::new();
    for (_index, c) in  line.chars().enumerate() {

      if c == '(' {
        expected.push(')');
      } else if c == '[' {
        expected.push(']');

      } else if c == '{' {
        expected.push('}');

      } else if c == '<' {
       expected.push('>');

      } else if &c != expected.get(expected.len() - 1).unwrap() {
        expected.clear(); //This is a change from Puzzle 1
        break;
      } else if &c == expected.last().unwrap() { //This was an add from puzzle 1
        expected.pop();
      }

    }
    

    //Find the remaining scores - Slight change in how scores are calculated
    if expected.len() > 0 {

      let mut i: i32 = (expected.len() -1) as i32; 

      while i >= 0 {

        score = score * 5;

        if expected.get(i as usize).unwrap() == &')' {
          score += 1; 
        } else if expected.get(i as usize).unwrap() == &']' {
          score += 2;
        } else if expected.get(i as usize).unwrap() == &'}' {
          score += 3;
        } else if expected.get(i as usize).unwrap() == &'>' {
          score += 4;
        }

        i -= 1; 
      }

      scores.push(score);
      score = 0;

    }

  }

  //Sort the Scores
  scores.sort();
  println!("Number of Scores: {}", scores.len());

  //Calculate the median score
  let median = scores.len() / 2;
  println!("Median Score Position: {}", median);

  //Return the median score
  score = *scores.get(median as usize).unwrap();
  score

}