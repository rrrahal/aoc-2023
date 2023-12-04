use std::{fs, os::unix::fs::DirEntryExt, result, collections::HashMap};

fn main() {
    let result_1 = part_1();
    println!("{result_1}");

    let result_2 = part_2();

    println!("{result_2}")
}

fn read_input() -> String {
  let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");

  println!("input: \n{contents}\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  println!("test: \n{contents}\n");

  return contents
}

fn parse_line(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut splitted = input.split(": ");
    let numbers: Vec<&str> = splitted.nth(1).unwrap().split(" | ").collect();

    let current_numbers = numbers.clone().into_iter().nth(0).unwrap();
    let lottery_numbers = numbers.clone().into_iter().nth(1).unwrap();


    let final_current_numbers: Vec<i32> = current_numbers.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();
    let final_loterry_numbers: Vec<i32> = lottery_numbers.split_whitespace().into_iter().map(|x| x.parse().unwrap()).collect();

    (final_current_numbers, final_loterry_numbers)
}

fn part_1() -> i32 {
  let input = read_input();
  let lines = input.lines();
  let mut result: i32 = 0;

  for line in lines {
    let mut found_words: Vec<i32> = vec![];
    let (mut current_numbers, mut lottery_numbers) = parse_line(&line.to_string());
    current_numbers.sort();
    lottery_numbers.sort();

    for number in &current_numbers {
        if let Ok(_) = lottery_numbers.binary_search(&number) {
            found_words.push(*number)
        }
    }

    if found_words.len() > 0 {
        result += 2_i32.pow((found_words.len() - 1).try_into().unwrap())
    }
  }
  result
}


fn part_2() -> i32 {
    let input = read_input();
    let lines = input.lines();
    let mut number_of_cards: HashMap<i32, i32> = HashMap::new();
    let mut result: i32 = 0;

    for (idx,line) in lines.enumerate() {
      let current_key = (idx+1) as i32;
      number_of_cards.insert(current_key, 0 + if number_of_cards.contains_key(&current_key) { number_of_cards[&current_key] } else { 1 });
      let mut found_numbers: Vec<i32> = vec![];
      let (mut current_numbers, mut lottery_numbers) = parse_line(&line.to_string());
      current_numbers.sort();
      lottery_numbers.sort();

      for number in &current_numbers {
          if let Ok(_) = lottery_numbers.binary_search(&number) {
            found_numbers.push(*number)
          }
      }


      for n in 0..number_of_cards[&current_key] {
        for i in 1..found_numbers.len()+1 {
            let k = (idx+1+i) as i32;
            number_of_cards.insert(k, 1 + if number_of_cards.contains_key(&k) { number_of_cards[&k] } else { 1 });
          }
      }
    }

    for entry in number_of_cards {
        result += entry.1
    }

    result
}
