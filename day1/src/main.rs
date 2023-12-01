use std::{fs, char, collections::HashMap, result};

struct Number {
    index: usize,
    value: i32,
}

fn main() {
    //let result_1 = part_1();
    //println!("{result_1}");

    let result_2 = part_2();

    println!("{result_2}")
}

fn read_input() -> String {
  let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");

  println!("input: {contents}");
  println!("\n\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  println!("test: \n {contents}");
  println!("\n\n");

  return contents
}

fn get_digits(input: String) -> i32 {
    let chars = input.chars();

    let mut start = -1;
    let mut end: Option<i32> = None;

    for c in chars {
        if c.is_digit(10) {
            if start == -1 {
                start = c.to_digit(10).unwrap() as i32;
            } else {
                end = Some(c.to_digit(10).unwrap() as i32);
            }
        }
    }

    if end == None {
        return start*10 + start;
    }

    return start*10 + end.unwrap();
}

fn part_1() -> i32 {
  let input = read_input();
  let lines = input.lines();

  let result = lines.map(|line| get_digits(line.to_string())).sum();

  result
}

fn get_number_word(input: &str) -> Option<i32> {
    if input.len() < 3 {
        return None;
    }

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for (i, digit) in digits.iter().enumerate() {
        if input.starts_with(*digit) {
            return Some((i + 1) as i32);
        }
    }

    None
}

fn get_written_digits(input: String) -> i32 {
    let mut chars = input.chars();

    let mut start: Option<i32> = None;
    let mut end: Option<i32> = None;

    let current_str = chars.as_str();
    let number = get_number_word(current_str);
    if let Some(number) = number {
        start = Some(number);
    }

    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            if start == None {
                start = Some(c.to_digit(10).unwrap() as i32);
            } else {
                end = Some(c.to_digit(10).unwrap() as i32);
            }
        }
        let current_str = chars.as_str();
        let number = get_number_word(current_str);
        if let Some(number) = number {
            if start == None {
                start = Some(number);
            } else {
                end = Some(number);
            }
        }
    }

    if end == None {
        return start.unwrap()*10 + start.unwrap();
    }
    start.unwrap()*10 + end.unwrap()

}


fn part_2() -> i32 {
  let input = read_input();
  let lines = input.lines();


  let result = lines.map(|line| get_written_digits(line.to_string()));

  result.sum()
}
