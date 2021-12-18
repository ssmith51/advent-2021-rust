use std::fs::File;
use std::io::{BufRead, BufReader};

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

//Using Structs like this removes builtin functionality (e.g. println!("{:?}") )
struct Command {
  direction: String,
  distance: i64,
}

fn main() {
  println!("Advent of Code - Day 01");
  let orders = read_input(FILE_NAME);
  let ol = orders.len();
  println!("Number of orders: {}", ol);

  println!("----------------------");
  println!("Starting Puzzle 1");
  let result = puzzle_1(&orders);
  println!("Total: {}", result);
  println!("----------------------");
  println!("Starting Puzzle 2");
  let result = puzzle_2(&orders);
  println!("Total: {}", result);

}


fn read_input(filename: &str) -> Vec<Command>{
  println!("Reading file: {}", filename);

  let mut commands: Vec<Command> = Vec::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  // Read the lines and create a Vec of Orders
  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap(); //This is how to read in a line

    let parts: Vec<&str> = line.split(" ").collect();
    let order = Command {
      direction: parts[0].to_string(), 
      distance: parts[1].to_string().parse().unwrap(), //A bit long winded, but converts &str -> String -> i64 
    };

    commands.push(order);
  }

  commands
}

fn puzzle_1(orders: &Vec<Command>) -> i64{

  let (mut h_pos, mut depth): (i64, i64) = (0,0);

  for order in orders {

    if order.direction.eq("forward") {
      h_pos = h_pos + order.distance;

    } else if order.direction.eq("up") {
      depth = depth - order.distance;

    } else if order.direction.eq("down"){
      depth = depth + order.distance;

    }
  }

  h_pos * depth

}

fn puzzle_2(orders: &Vec<Command>) -> i64{

  let (mut h_pos, mut depth, mut aim): (i64, i64, i64) = (0,0, 0);

  for order in orders {

    if order.direction.eq("forward") {
      h_pos = h_pos + order.distance;
      depth = depth + (aim * order.distance);

    } else if order.direction.eq("up") {
      aim = aim - order.distance;

    } else if order.direction.eq("down"){
      aim = aim + order.distance;

    }
  }

  h_pos * depth

}