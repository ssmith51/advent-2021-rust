
fn main() {
    println!("Advent of Code - Day 07");

  //Read in the test file from the root dir as a string
  let report = include_str!("../test.txt");

  let readings: Vec<_> = report.lines().collect();

  for v in readings {
      println!("{}",v);
  }
}
