use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter::FromIterator;

//Gobal file name for quick change
static FILE_NAME: &str = "input.txt";

fn main() {
  println!("Advent of Code - Day 08");
  let input = read_input(FILE_NAME);

  println!("----------------------");
  println!("Starting Puzzle 1");
  let result = puzzle_1(&input);
  println!("Total count: {}", result);
  println!("----------------------");
  println!("Starting Puzzle 2");
  let result = puzzle_2(&input);
  println!("Total count: {}", result);

}

fn read_input(filename: &str) -> Vec<String> {

  let mut readings: Vec<String> = Vec::new();

  let fi = File::open(filename).unwrap();
  let reader = BufReader::new(fi);

  for (_index, line) in reader.lines().enumerate() {
    let line = line.unwrap();
    readings.push(line.to_string())
  }

  readings

}

fn puzzle_1(readings: &Vec<String>) -> i64 {
  // let readings = readings.to_vec(); // Create a copy of Readings
  let mut count: i64 = 0;

  for val in readings {
    let part: Vec<&str> = val.split("|").collect();
    let parts: Vec<&str> = part[1].split(" ").collect();
    for p in parts {
      let l: usize = p.len();
      if l == 2 || l == 4 || l == 3 || l == 7 {
        count += 1
      }
    }
  }
  count
}

/*
Here is the Logic to determine what number is what. 

1, 4, 7 , 8 are known
Top Line = difference between 7-1
Top Left & Middle Lines = difference 4 - 1
Bottom Left & Bottom Lines = difference of 8-4 - Top Line


2 = len(5) w/Top Line, Bottom Left & Bottom Line
5 = len(5) w/Top Line, Top Left & Middle
3 = Last len(5) left
0 = len(6) w/Top Line, Right Lines, Top Line, Bottom Left & Bottom Lines, 1 of Top Right and Middle
6 = len(6) w/Top Line, Top Left & Middle, Bottom Left & Bottom, 1 of Right Lines
9 = len(6) left
*/
fn puzzle_2(readings: &Vec<String>) -> i64 {
  let mut total: i64 = 0;

 
  for val in readings {
    let parts: Vec<&str> = val.split("|").collect();
    let nums: Vec<&str> = parts[0].split(" ").collect();

    //Known Numbers
    let one = nums.iter().find(|val| val.len() == 2).unwrap().to_string();
    let four = nums.iter().find(|val| val.len() == 4).unwrap().to_string();
    let seven = nums.iter().find(|val| val.len() == 3).unwrap().to_string();
    let eight = nums.iter().find(|val| val.len() == 7).unwrap().to_string();
    
    //Numbers to calculate
    let mut two: String = "".to_string();
    let mut three: String = "".to_string();
    let mut five: String = "".to_string();
    let mut six: String = "".to_string();
    let mut nine: String = "".to_string();
    let mut zero: String = "".to_string();

    
    //Find the Top, Left, Middle, Bottom Left and Bottom line possibilities
    let top_line: String = find_difference(&seven, &one);
    println!("Top Line: {}", top_line);

    let top_left_middle = find_difference(&four, &one);
    println!("Top Left or Middle: {}", top_left_middle);

    let bottom_left_bottom = find_difference(&eight, &four);
    let bottom_left_bottom = find_difference(&bottom_left_bottom, &top_line);
    println!("Bottom Left orr Bottom: {}", &bottom_left_bottom);

    //Convert One, Bottom Left of Bottom and Top Left or Middle to Vecs for easier comparisons
    let o: Vec<char> = one.chars().collect();
    let blb: Vec<char> = bottom_left_bottom.chars().collect();
    let tlm: Vec<char> = top_left_middle.chars().collect();
   
    //Calculate the hard to find numbers
    for num in nums {

      // Find 2, 3 5
      if num.len() == 5 {
        
        //5 = len(5) w/Top Line, Top Left & Middle Lines
        if num.contains(&top_line[..]) && num.contains(tlm[0]) && num.contains(tlm[1]) {
          five = num.to_string();

        //2 = len(5) w/Top Line, Bottom Left & Bottom Line
        } else if num.contains(&top_line[..]) && num.contains(blb[0]) && num.contains(blb[1]) {
          two = num.to_string();
        
          //3 = Last len(5) left
        } else {
          three = num.to_string();
        }

      //Find 0, 6, 9
      } else if num.len() == 6 { 

        //0 = len(6) w/Top Line, Right Lines, Top Line, Bottom Left & Bottom Lines, 1 of Top Right and Middle
        if num.contains(&top_line[..]) && num.contains(o[0]) && num.contains(o[1]) && num.contains(blb[0]) && num.contains(blb[1]) {
          if num.contains(tlm[0]) && !num.contains(tlm[1]) {
            zero = num.to_string();

          } else if num.contains(tlm[1]) && !num.contains(tlm[0]) {
            zero = num.to_string();
          }

        //6 = len(6) w/Top Line, Top Left & Middle, Bottom Left & Bottom, 1 of Right Lines
        } else if num.contains(&top_line[..]) && num.contains(tlm[0]) && num.contains(tlm[1]) && num.contains(blb[0]) && num.contains(blb[1]) {
          if num.contains(o[0]) && !num.contains(o[1]) {
            six = num.to_string();

          } else if num.contains(o[1]) && !num.contains(o[0]) {

            six = num.to_string();
          }
        
        //9 = len(6) left
        } else {
          nine = num.to_string();
        }

      }
    }

    println!("Number Mappings:");
    println!(" One: {}\n Two: {}\n Three: {}\n Four: {}\n Five: {}\n Six: {}\n Seven: {}\n Eight: {}\n Nine: {}\n Zero: {}", one, two, three, four, five, six, seven, eight, nine, zero);
    println!("");
    
    //Read the input from after the | 
    let reading: Vec<&str> = parts[1].split(" ").collect();

    //Covert each number to a vector for easier comparison
    let one: Vec<char> = one.chars().collect();
    let two: Vec<char> = two.chars().collect();
    let three: Vec<char> = three.chars().collect();
    let four: Vec<char> = four.chars().collect();
    let five: Vec<char> = five.chars().collect();
    let six: Vec<char> = six.chars().collect();
    let seven: Vec<char> = seven.chars().collect();
    let eight: Vec<char> = eight.chars().collect();
    let nine: Vec<char> = nine.chars().collect();
    let zero: Vec<char> = zero.chars().collect();

    //Holds the running value from the input 
    let mut val: String = "".to_string();

    for num in reading {

      //Convert num from &str to String for contains function
      let num: String = num.to_string();

      //Caculate the number from the code by comparing every letter in the code with the letters in the number
      if num.len() > 1 {
        if num.contains(one[0]) && num.contains(one[1]) && num.len() == 2 {
          val.push('1');
        } else if num.contains(two[0]) && num.contains(two[1]) && num.contains(two[2]) && num.contains(two[3]) && num.contains(two[4]) && num.len() == 5 {
          val.push('2');
        } else if num.contains(three[0]) && num.contains(three[1]) && num.contains(three[2]) && num.contains(three[3]) && num.contains(three[4]) && num.len() == 5 {
          val.push('3');
        } else if num.contains(four[0]) && num.contains(four[1]) && num.contains(four[2]) && num.contains(four[3]) && num.len() == 4 {
          val.push('4');
        } else if num.contains(five[0]) && num.contains(five[1]) && num.contains(five[2]) && num.contains(five[3]) && num.contains(five[4])  && num.len() == 5{
          val.push('5');
        } else if num.contains(six[0]) && num.contains(six[1]) && num.contains(six[2]) && num.contains(six[3]) && num.contains(six[4]) && num.contains(six[5])  && num.len() == 6 {
          val.push('6');
        } else if num.contains(seven[0]) && num.contains(seven[1]) && num.contains(seven[2]) && num.len() == 3 {
          val.push('7');
        } else if num.contains(eight[0]) && num.contains(eight[1]) && num.contains(eight[2]) && num.contains(eight[3]) && num.contains(eight[4]) && num.contains(eight[5]) && num.contains(eight[6])  && num.len() == 7 {
          val.push('8');
        } else if num.contains(nine[0]) && num.contains(nine[1]) && num.contains(nine[2]) && num.contains(nine[3]) && num.contains(nine[4]) && num.contains(nine[5])  && num.len() == 6 {
          val.push('9');
        }else if num.contains(zero[0]) && num.contains(zero[1]) && num.contains(zero[2]) && num.contains(zero[3]) && num.contains(zero[4]) && num.contains(zero[5])  && num.len() == 6 {
          val.push('0');
        } 
  
      }
      
    }
    println!("Reading Value: {}", val);

    //Convert the read value to a int
    let calc: i64 = val.parse().unwrap();

    //Create a running total of the vaules
    total += calc;

  }

  total
}


//Set comparison to find what characters are not included in the second variable passed in
fn find_difference(left: &String, right: &String)  -> String {
  let mut a: HashSet<char> = HashSet::new();
  for c in left.chars() {
    a.insert(c);
  }

  let mut b: HashSet<char> = HashSet::new();
  for c in right.chars() {
    b.insert(c);
  }

  let diff: Vec<&char> = a.difference(&b).collect();
  let diff = String::from_iter(diff);
  diff

}