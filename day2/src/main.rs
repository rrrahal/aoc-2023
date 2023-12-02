use std::{fs, result};

#[derive(Debug)]
struct Round {
  id: i32,
  games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
  red: i32,
  blue: i32,
  green: i32,
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

  println!("input:\n{contents}\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  println!("input:\n{contents}\n");

  return contents
}

fn parse_input(input: String) -> Round {
  let mut game_id_split = input.split(": ");
  let id = game_id_split.nth(0).unwrap().split(" ").nth(1).unwrap();
  let games = game_id_split.nth(0).unwrap().split("; ");

  let mut round = Round {
    id: id.parse::<i32>().unwrap(),
    games: Vec::new(),
  };

  for game in games {
    let mut game_ds = Game {
      red: 0,
      blue: 0,
      green: 0,
    };
    let colors_pair = game.split(", ");
    for color in colors_pair {
      let mut color_split = color.split(" ");
      let color_number = color_split.nth(0).unwrap().parse::<i32>().unwrap();
      let color_name = color_split.nth(0).unwrap();
      match color_name {
        "red" => game_ds.red = color_number,
        "blue" => game_ds.blue = color_number,
        "green" => game_ds.green = color_number,
        _ => println!("Unknown color"),
      }
    }
    round.games.push(game_ds);
  }

  round
}

fn part_1() -> i32 {
  let input = read_input();
  let lines = input.lines();
  let mut result = Vec::new();

  for line in lines {
    let round = parse_input(line.to_string());
    let mut games_iter = round.games.iter();
    let impossible_games = games_iter.any(|game| game.blue > 14 || game.red > 12 || game.green > 13);
    if !impossible_games {
      //println!("Round {} is possible", round.id);
      result.push(round.id);
    }
  }
  result.iter().sum()
}


fn part_2() -> i32 {
  let input = read_input();
  let lines = input.lines();
  let mut result: Vec<i32> = Vec::new();

  for line in lines {
    let round = parse_input(line.to_string());
    let games_iter = round.games.iter();
    let min_green = games_iter.clone().map(|game| {if game.green > 0 {game.green} else {1}}).max().unwrap();
    let min_red = games_iter.clone().map(|game| {if game.red > 0 {game.red} else {1}}).max().unwrap();
    let min_blue = games_iter.clone().map(|game| {if game.blue > 0 {game.blue} else {1}}).max().unwrap();

    let power = min_blue*min_green*min_red;
    //println!("Round {} has power {}", round.id, power);
    result.push(power);
    }
  result.iter().sum()
}
