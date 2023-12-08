use std::{fs, collections::HashMap};


struct Pattern {
  chars: Vec<char>
}

impl Pattern {
    fn get(&self, pos: i32) -> char {
      let len = self.chars.len() as i32;
      let index = pos % len;

      return self.chars[index as usize]
    }
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

  println!("test:\n{contents}\n");

  return contents
}

fn parse_line(line: String) -> (String, String, String) {
    let splitted = line.split(" = ");
    let first_node = splitted.clone().nth(0).unwrap().to_string();
    let mut second_node = splitted.clone().nth(1).unwrap().split(", ").nth(0).unwrap().to_string();
    let mut third_node = splitted.clone().nth(1).unwrap().split(", ").nth(1).unwrap().to_string();
    third_node.pop();
    second_node.remove(0);

    //println!("{:?} - {:?} - {:?}", first_node, second_node, third_node);

    (first_node, second_node, third_node)
}

fn part_1() -> i32 {
  let input = read_input();
  let mut lines = input.lines();
  let pattern = Pattern { chars: lines.next().unwrap().chars().collect()};
  let mut current_place = &"AAA".to_string();
  let mut i = 0;

  let mut nodes: HashMap<String, (String, String)> = HashMap::new();

  for line in lines {
    if line != "" {
        let (first, second, third) = parse_line(line.to_string());
        nodes.insert(first, (second, third));
    }
  }

  while current_place != &"ZZZ".to_string() {
      let current_node = nodes.get(current_place).unwrap();
      let mov = pattern.get(i);
      if mov == 'R' {
        current_place = &current_node.1;
      } else {
        current_place = &current_node.0;
      }
      i += 1;
  }
  i
}

fn get_part_1(start_pos: &String, pattern: &Pattern, nodes: &HashMap<String, (String, String)>) -> Vec<i32> {
  let mut i = 0;
  let mut current_place = start_pos;
  let mut results: Vec<i32> = vec![];

  let mut seen_moves: Vec<(char, &String, i32)> = vec![];
  let mut step = 0;

  loop {
    let current_mov = pattern.get(i);
    if seen_moves.contains(&(current_mov, current_place, i)) {
      println!("\n");
      return results
    }

    seen_moves.push((current_mov, current_place, i));

    if current_mov == 'R' {
      current_place = &nodes.get(current_place).unwrap().1;
    } else {
      current_place = &nodes.get(current_place).unwrap().0;
    }

    if current_place.ends_with("Z") {
      results.push(step + 1);
    }
    if i + 1 == pattern.chars.len() as i32 {
      i = 0
    } else {
      i += 1
    }

    step += 1;

  }
}


fn part_2() -> i32 {
  let input = read_input();
  let mut lines = input.lines();
  let pattern = Pattern { chars: lines.next().unwrap().chars().collect()};
  let mut nodes: HashMap<String, (String, String)> = HashMap::new();

  let mut idxs: Vec<i32> = vec![];

  for line in lines {
    if line != "" {
        let (first, second, third) = parse_line(line.to_string());
        nodes.insert(first, (second, third));
    }
  }
  let current_places: Vec<&String> = nodes.keys().filter(|x| x.ends_with("A")).collect();

  for place in current_places {
    let mut results = get_part_1(place, &pattern, &nodes);
    println!("{:?}", results);
    idxs.append(&mut results);
  }

  println!("idxs: {:?} ", idxs);

  0
}