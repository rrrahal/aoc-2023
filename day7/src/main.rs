use std::{fs, collections::HashMap, cmp::Ordering};

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn get_power(&self) -> i32 {
      match *self {
          HandType::HighCard => 0,
          HandType::OnePair => 1,
          HandType::TwoPair => 2,
          HandType::ThreeOfAKind => 3,
          HandType::FullHouse => 4,
          HandType::FourOfAKind => 5,
          HandType::FiveOfAKind => 6,
      }
    }

    fn get_type(i: i32) -> Self {
      match i {
          0 => Self::HighCard,
          1 => Self::OnePair,
          2 => Self::TwoPair,
          3 => Self::ThreeOfAKind,
          4 => Self::FullHouse,
          5 => Self::FourOfAKind,
          6 => Self::FiveOfAKind,
          _ => Self::FiveOfAKind
      }
    }
}

#[derive(Debug)]
struct Card {
    label: char,
}

impl Card {
    fn new(label: char) -> Self {
        Card { label }

    }

    fn get_str(&self) -> i32 {
      let l = self.label;
        match l {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => 0
        }
    }

    fn get_str_j(&self) -> i32 {
      let l = self.label;
        match l {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => -1,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => 0
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: i32
}

impl Hand {
    fn new() -> Self {
      Hand {
        cards: vec![],
        hand_type: HandType::TwoPair,
        bid: 0
      }
    }

    fn change_type(mut self) -> Self {
      let cards = &self.cards;
      let mut map: HashMap<char,i32> = HashMap::new();

      for card in cards {
        map.entry(card.label).and_modify(|x| *x += 1).or_insert(1);
      }

      if map.len() == 1 {
        self.hand_type = HandType::FiveOfAKind;
      }

      if map.len() == 5 {
        self.hand_type = HandType::HighCard;
      }

      if map.len() == 4 {
        self.hand_type = HandType::OnePair;
      }

      if map.len() == 3 {
        for (_, n) in &map {
          if *n == 2 {
            self.hand_type = HandType::TwoPair;
            return self
          }
        }

        self.hand_type = HandType::ThreeOfAKind;
      }

      if map.len() == 2 {
        for (_, n) in &map {
          if *n == 4 {
            self.hand_type = HandType::FourOfAKind;
            return self;
          }
        }
        self.hand_type = HandType::FullHouse;
      }
      return self
    }

    fn change_type_j(mut self) -> Self {
      let cards = &self.cards;
      let number_of_js = cards.into_iter().filter(|x| x.label == 'J').count() as i32;

      if number_of_js == 0 {
        return self.change_type();
      }

      if number_of_js == 5 {
        self.hand_type = HandType::FiveOfAKind;
        return self
      }

      let mut map: HashMap<char,i32> = HashMap::new();

      for card in cards {
        if card.label == 'J' {
          continue
        }
        map.entry(card.label).and_modify(|x| *x += 1).or_insert(1);
      }

      if map.len() == 4 {
        self.hand_type = HandType::OnePair;
      }

      if map.len() == 3 {
        match number_of_js {
            1 => {
              if map.values().any(|x| *x == 2) {
                self.hand_type = HandType::ThreeOfAKind;
              } else {
                self.hand_type = HandType::TwoPair;
              }
            },
            2 => self.hand_type = HandType::ThreeOfAKind,
            _ => {}

        }
      }

      if map.len() == 2 {
        match number_of_js {
            1 => {
              if map.values().any(|x| *x == 3) {
                self.hand_type = HandType::FourOfAKind;
              } else {
                self.hand_type = HandType::FullHouse;
              }
            },
            2 => self.hand_type = HandType::FourOfAKind,
            3 => self.hand_type = HandType::FourOfAKind,
            _ => {}
        }
      }

      if map.len() == 1 {
        self.hand_type = HandType::FiveOfAKind;
      }

      return self
    }
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

  println!("input:\n{contents}\n");

  return contents
}

fn read_test() -> String {
  let contents = fs::read_to_string("./test.txt")
    .expect("Should have been able to read the file");

  println!("test:\n{contents}\n");

  return contents
}

fn part_1() -> i32 {
    let input = read_input();
    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
      let mut hand = Hand::new();
      let card = line.split_whitespace().nth(0).unwrap();
      for c in card.chars() {
        hand.cards.push(Card { label: c });
      }

      let bid: i32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();

      //hand = hand.change_type();
      hand.bid = bid;

      hands.push(hand);
    }

    hands.sort_by(|a, b| {
      if a.hand_type == b.hand_type {
        for i in 0..a.cards.len() {
          let v_a = a.cards[i].get_str();
          let v_b = b.cards[i].get_str();
          if v_a != v_b {
            if v_a > v_b {
              return Ordering::Greater
            } else {
              return Ordering::Less
            }
          }
        }
      } else {
        if a.hand_type.get_power() > b.hand_type.get_power() {
          return Ordering::Greater
        } else {
          return Ordering::Less
        }
      }

      Ordering::Equal

    });

    for hand in &hands {
      println!("{:?}", hand);
    }

    hands.into_iter().enumerate().fold(0, |acc, (idx, hand) | {
      acc + ((idx + 1) as i32) * hand.bid
    })
}


fn part_2() -> i32 {
  let input = read_input();
    let mut hands: Vec<Hand> = vec![];

    for line in input.lines() {
      let mut hand = Hand::new();
      let card = line.split_whitespace().nth(0).unwrap();
      for c in card.chars() {
        hand.cards.push(Card::new(c));
      }

      let bid: i32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();

      hand = hand.change_type_j();
      hand.bid = bid;

      hands.push(hand);
    }

    hands.sort_by(|a, b| {
      if a.hand_type == b.hand_type {
        for i in 0..a.cards.len() {
          let v_a = a.cards[i].get_str_j();
          let v_b = b.cards[i].get_str_j();
          if v_a != v_b {
            if v_a > v_b {
              return Ordering::Greater
            } else {
              return Ordering::Less
            }
          }
        }
      } else {
        if a.hand_type.get_power() > b.hand_type.get_power() {
          return Ordering::Greater
        } else {
          return Ordering::Less
        }
      }

      Ordering::Equal

    });

    for hand in &hands {
      for card in &hand.cards {
        if card.label == 'J' {
          let cards_str = hand.cards.iter().map(|x| x.label).collect::<String>();
          let hand_type = &hand.hand_type;
          println!("{:?} - {:?}", cards_str, hand_type);
        }
      }
    }

    hands.into_iter().enumerate().fold(0, |acc, (idx, hand) | {
      acc + ((idx + 1) as i32) * hand.bid
    })
}