use std::fs::File;
use std::io::{BufRead, BufReader};
// use regex::Regex;

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

fn main() {
    println!("Advent of Code - Day 10");
    let input = read_input(FILE_NAME);
    println!("----------------------");
    println!("Starting Puzzle 1");
    let result = puzzle_1(&input);
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

    if illegal == ')' {
      score += 3; 
    } else if illegal == ']' {
      score += 57;
    } else if illegal == '}' {
      score += 1197;
    } else if illegal == '>' {
      score += 25137;
    }





    // let mut test: String = line.clone();

    // let mut matched = re.is_match(&line[..]);

    // let mut removed: i64 = 0;
    // let mut illegal_char: char = '1';

    // while matched {

    //   let capture = re.captures(&test).unwrap().get(1).unwrap().as_str();
    //   let caps: Vec<char> = capture.chars().collect();
    //   let mut temp: String = "".to_string();

    //   for (_i, c) in line.chars().enumerate() {

    //     if c != caps[0] && c != caps[1] && removed < 2{
    //       illegal_char = c;
    //       temp.push(c);
    //       removed += 1;
    //     } 
    //   }

    //   matched = re.is_match(&temp[..]);
    //   test = temp;

    //   println!("Test String: {}", test);

    // }

    // if test.len() > 0 {
    //   let c: Vec<char> = test.chars().collect();
    //   if c[0] != c[1]{
    //     println!("Final: {}", illegal_char);
    //     if illegal_char == ')' {
    //       score += 3; 
    //     } else if illegal_char == ']' {
    //       score += 57;
    //     } else if illegal_char == '}' {
    //       score += 1197;
    //     } else if illegal_char == '>' {
    //       score += 25137;
    //     }
    //   }
    // }
  }
  
  // let mut test: String = "[<>({}){}[([)]<>]]".to_string();

  
  // let mut matched = re.is_match(&test[..]);
  // println!("Matched: {}",re.is_match(&test[..]));
  // let mut removed: i64 = 0;
  // let mut illegal_char: char = '1';
  // while matched {

  //   let capture = re.captures(&test).unwrap().get(1).unwrap().as_str();
  //   let caps: Vec<char> = capture.chars().collect();
  //   let mut temp: String = "".to_string();
  //   for (_i, c) in test.chars().enumerate() {

  //     if c != caps[0] && c != caps[1] && removed < 2{
  //       temp.push(c);
  //       removed += 1;
  //     } else if removed == 2 {
  //       illegal_char = c;
  //     }
  //   }

  //   test = temp;
  //   // println!("Test: {}", test);
  //   matched = re.is_match(&test[..]);
  //   // println!("Re-Matched: {}", matched);
  // }

  // if test.len() > 0 {
  //   println!("Final: {}", illegal_char);
  //   if illegal_char == ')' {
  //     score += 3; 
  //   } else if illegal_char == ']' {
  //     score += 57;
  //   } else if illegal_char == '}' {
  //     score += 1197;
  //   } else if illegal_char == '>' {
  //     score += 25137;
  //   }
  // }

  score

}