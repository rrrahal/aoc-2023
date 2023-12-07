use std::fs;
use roots::Roots;
use roots::find_roots_quadratic;

#[derive(Debug)]
struct Race {
    time: f64,
    record: f64,
}

fn main() {
    let result_1 = part_1();
    println!("{result_1}");

    let result_2 = part_2();

    println!("{result_2}")
}

fn read_input() -> String {
  let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");

  println!("input:\n{contents}\n\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  println!("test:\n{contents}\n\n");

  return contents
}

fn parse_input(input: String) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split(": ").nth(1).unwrap().split_whitespace();
    let mut result = vec![];
    for time in times {
        let time_number: u32 = time.parse().unwrap();
        result.push(Race {time: time_number as f64, record: 0.0});
    }

    let records = lines.next().unwrap().split(": ").nth(1).unwrap().split_whitespace();
    for (idx, record) in records.enumerate() {
        let record_number = record.parse().unwrap();
        result[idx].record = record_number;
    }
    result
}

fn find_roots(race: Race) -> Option<(f64, f64)> {
    let b: f64 = race.time as f64;
    let c = race.record as f64;
    let roots = find_roots_quadratic(-1 as f64, b, -(c));

    match roots {
        Roots::No(_) => None,
        Roots::One(_) => None,
        Roots::Two(x) => Some((x[0], x[1])),
        _ => None
    }
}

fn part_1() -> f64 {
  let input = read_input();
  let races = parse_input(input);
  let mut results: Vec<f64> = vec![];

  for race in races {
    if let Some((first_root, second_root)) = find_roots(race) {
        println!("{:?} - {:?}", first_root, second_root);
        results.push((second_root.floor() + - first_root.ceil()) + 1.0);
    }
  }

  println!("{:?}", results);

  results.into_iter().fold(1 as f64,|acc, x| x * acc)
}


fn part_2() -> f64 {
    let input = read_input();
    let mut lines = input.lines();
    let time = lines.next().unwrap().split(": ").nth(1).unwrap().split_whitespace().fold("".to_string(), |acc, x| acc + x);
    let record_str = lines.next().unwrap().split(": ").nth(1).unwrap().split_whitespace().fold("".to_string(), |acc, x| acc + x);

    println!("strs -> {:?} - {:?}", time, record_str);
    if let Some((first_root, second_root)) = find_roots(Race { time: time.parse().unwrap(), record: record_str.parse().unwrap() }) {
        println!("{:?} - {:?}", first_root, second_root);
        return ((second_root.floor()) - first_root.ceil()) + 1.0
    }
    1.0
}