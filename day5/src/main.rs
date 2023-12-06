use std::fs;
use std::ops::Range;
use std::str::Lines;

// Struct Seeds -> holds all values for a seed
// Map with id -> Seed
// Fill all seeds (that are interesting)

#[derive(Debug)]
struct Seed {
    id:u64,
    soil:u64,
    fert:u64,
    water:u64,
    light:u64,
    temp:u64,
    hum:u64,
    loc:u64
}

impl Seed {
    fn new(id: u64) -> Self {
        Seed { id, soil: id, fert: id, water: id, light: id, temp: id, hum: id, loc: id }
    }
}

#[derive(Debug)]
struct SeedRange {
    start: u64,
    length: u64
}



fn main() {
    let result_1 = part_1();
    println!("{result_1}");

    let result_2 = part_2();

    //println!("{result_2}")
}

fn read_input() -> String {
  let contents = fs::read_to_string("./input.txt")
    .expect("Should have been able to read the file");

  //println!("input: \n{contents}\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  //println!("test: \n{contents}\n");

  return contents
}

fn parse_range(line: &str) -> (Range<u64>, Range<u64>) {
    let numbers: Vec<u64> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let end_range = numbers[0];
    let start_range = numbers[1];
    let size = numbers[2];

    let start = start_range..start_range+size;
    let end = end_range..end_range+size;

    return (start, end)

}

fn parse_input(input: String) -> Vec<Seed> {
    let mut lines = input.lines();
    let mut seeds: Vec<Seed> = vec![];
    while let Some(mut current_item) = lines.next()  {
        if current_item.starts_with("seeds: ") {
            let seeds_ids: Vec<u64> = current_item.split("seeds: ").nth(1).unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            for seed in seeds_ids {
                seeds.push(Seed::new(seed));
            }
        }

        else if current_item.starts_with("seed-to-soil") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.id) {
                        let offset = seed.id - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.soil = new_v;
                        seed.fert = new_v;
                        seed.water = new_v;
                        seed.light = new_v;
                        seed.temp = new_v;
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("soil-to-fertilizer") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.soil) {
                        let offset = seed.soil - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.fert = new_v;
                        seed.water = new_v;
                        seed.light = new_v;
                        seed.temp = new_v;
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("fertilizer-to-water") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.fert) {
                        let offset = seed.fert - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.water = new_v;
                        seed.light = new_v;
                        seed.temp = new_v;
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("water-to-light") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.water) {
                        let offset = seed.water - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.light = new_v;
                        seed.temp = new_v;
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("light-to-temperature") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.light) {
                        let offset = seed.light - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.temp = new_v;
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("temperature-to-humidity") {
            current_item = lines.next().unwrap();
            while current_item != ""  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.temp) {
                        let offset = seed.temp - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.hum = new_v;
                        seed.loc = new_v;

                    }
                }
                current_item = lines.next().unwrap();
            }
        }

        else if current_item.starts_with("humidity-to-location") {
            while let  Some(current_item) = lines.next()  {
                let (start_range, end_range) = parse_range(current_item);
                for seed in &mut seeds {
                    if start_range.contains(&seed.hum) {
                        let offset = seed.hum - start_range.clone().nth(0).unwrap();
                        let new_v = end_range.clone().nth(offset as usize).unwrap();
                        seed.loc = new_v;

                    }
                }
            }
        }
    }

    seeds
}

fn part_1() -> u64 {
  let input = read_input();
  let seeds = parse_input(input);


  let mut min = seeds[0].loc;

    for seed in &seeds {
        if seed.loc < min {
            min = seed.loc
        }
    };

  min
}

fn get_seeds(seed_line: &str) -> Vec<SeedRange> {
    let mut seeds_ranges:  Vec<SeedRange> = vec![];
    let mut ranges = seed_line.split("seeds: ").nth(1).unwrap().split_whitespace();
    while let Some(current_element) = ranges.next()  {
        let start: u64 = current_element.parse().unwrap();
        let size: u64 = ranges.next().unwrap().parse().unwrap();
        seeds_ranges.push(SeedRange { start, length: size })
    }

    seeds_ranges
}


fn part_2() -> u64 {
    let input = read_test();
    let mut lines = input.lines();
    let seeds_range = get_seeds(lines.next().unwrap());
    let mut min = seeds_range[0].start;
    for seed in &seeds_range {
        if seed.start < min {
            min = seed.start
        }
    }

    println!("{:?}", seeds_range);

    min
}