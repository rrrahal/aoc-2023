use core::num;
use std::{fs, result};

// Number: Value: i32, position: (start_x, end_x,y)
#[derive(Debug, Clone, Copy)]
struct Number {
  value: i32,
  position: (i32, i32, i32)
}
#[derive(Debug)]
struct Symbol {
    value: char,
    position: (i32, i32)
}

// symbols: position: (x,y), value: char

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

fn check_coordinates(symbol_coord: (i32, i32), number_coord: (i32,i32,i32)) -> bool {
    let symbol_x = symbol_coord.0;
    let symbol_y = symbol_coord.1;

    let start_x = number_coord.0;
    let end_x = number_coord.1;
    let number_y = number_coord.2;

    let mut possible_pos: Vec<(i32,i32)> = vec![];
    possible_pos.push((start_x - 1, number_y));
    possible_pos.push((end_x+1, number_y));
    possible_pos.push((start_x-1, number_y+1));
    possible_pos.push((start_x-1, number_y-1));
    possible_pos.push((end_x+1, number_y+1));
    possible_pos.push((end_x+1, number_y-1));

    let yo = start_x..end_x+1;
    for num in yo {
        possible_pos.push((num, number_y+1));
        possible_pos.push((num, number_y-1));
    }


    return possible_pos.into_iter().any(|x| x == (symbol_x, symbol_y));

 }

fn get_missing_parts(symbols: Vec<Symbol>, numbers: Vec<Number>) -> Vec<Number> {
    let mut true_numbers: Vec<Number> = Vec::new();
    let mut found = false;
    for num in numbers {
        for sym in &symbols {
            if check_coordinates(sym.position, num.position) {
                found = true;
            }
        }

        if found {
            true_numbers.push(num)
        }
        found = false;
    }
    return true_numbers
}

fn has_symbol_adjacent(num: &Number, symbols: &Vec<Symbol>) -> bool {
    let start_x = num.position.0;
    let end_x = num.position.1;
    let number_y = num.position.2;
    for sym in symbols {
        let sym_x = sym.position.0;
        let sym_y = sym.position.1;

        if sym_y == number_y || sym_y == number_y + 1 || sym_y == number_y -1 {
            if sym_x >= start_x - 1 && sym_x <= end_x + 1 {
                return true
            }
        }
    }
    return false
}

fn part_1() -> i32 {
    let input = read_input();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let lines = input.lines();

    for (idx, line) in lines.enumerate() {
        let y = idx as i32;
        let mut start_x = None;
        let mut end_x = None;
        let mut value = None;

        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if let None = start_x {
                    start_x = Some(x as i32);
                    end_x = Some(x as i32);
                    value = Some(char.to_digit(10).unwrap() as i32);
                } else {
                    end_x = Some(x as i32);
                    value = Some(value.unwrap() * 10 + char.to_digit(10).unwrap() as i32);
                }
            } else {
                if let Some(current_x) = start_x {
                    numbers.push(Number { value: value.unwrap(), position: (current_x, end_x.unwrap(), y) });
                    start_x = None;
                    end_x = None;
                    value = None;
                }
            }
        }
        if let Some(current_x) = start_x {
            numbers.push(Number { value: value.unwrap(), position: (current_x, end_x.unwrap(), y) });
            start_x = None;
            end_x = None;
            value = None;
        }

        for (x, char) in line.chars().enumerate() {
            if !char.is_digit(10) && char != '.' {
                symbols.push(Symbol { value: char, position: (x as i32, y) })
            }
        }
    }

    let mut results: Vec<Number> = vec![];
    println!("{:?}", numbers);

    for num in numbers {
        if has_symbol_adjacent(&num, &symbols) {
            results.push(num)
        }
    }


    results.into_iter().map(|x| x.value).sum()
}



fn get_adjacent_numbers(gear: &Symbol, numbers: &Vec<Number>) -> Vec<Number> {
    let mut results: Vec<Number> = vec![];
    let gear_x = gear.position.0;
    let gear_y = gear.position.1;
    for num in numbers {
        let start_x = num.position.0;
        let end_x = num.position.1;
        let num_y = num.position.2;

        if gear_y == num_y || gear_y == num_y + 1 || gear_y == num_y -1 {
            if gear_x >= start_x - 1 && gear_x <= end_x + 1 {
                results.push(num.clone())
            }
        }
    }
    if results.len() == 2 {
        return results
    }

    return vec![]
}

fn part_2() -> i32 {
    let input = read_input();
    let mut numbers: Vec<Number> = Vec::new();
    let mut gears: Vec<Symbol> = Vec::new();

    let lines = input.lines();

    for (idx, line) in lines.enumerate() {
        let y = idx as i32;
        let mut start_x = None;
        let mut end_x = None;
        let mut value = None;

        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                if let None = start_x {
                    start_x = Some(x as i32);
                    end_x = Some(x as i32);
                    value = Some(char.to_digit(10).unwrap() as i32);
                } else {
                    end_x = Some(x as i32);
                    value = Some(value.unwrap() * 10 + char.to_digit(10).unwrap() as i32);
                }
            } else {
                if let Some(current_x) = start_x {
                    numbers.push(Number { value: value.unwrap(), position: (current_x, end_x.unwrap(), y) });
                    start_x = None;
                    end_x = None;
                    value = None;
                }
            }
        }
        if let Some(current_x) = start_x {
            numbers.push(Number { value: value.unwrap(), position: (current_x, end_x.unwrap(), y) });
            start_x = None;
            end_x = None;
            value = None;
        }

        for (x, char) in line.chars().enumerate() {
            if !char.is_digit(10) && char == '*' {
                gears.push(Symbol { value: char, position: (x as i32, y) })
            }
        }
    }

    let mut results: Vec<Vec<Number>> = vec![];
    for gear in gears {
        results.push(get_adjacent_numbers(&gear, &numbers))
    }

    let mut final_value: i32 = 0;

    for values in results.into_iter().filter(|x| x.len() == 2) {
        final_value += values.get(0).unwrap().value * values.get(1).unwrap().value
    }

    final_value
}
