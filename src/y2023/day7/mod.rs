#![allow(dead_code)]

use std::collections::HashMap;

use crate::read::read_input_file;

fn get_card_strength(card: &char) -> u32 {
      // WITH JOKER
      match card {
        'J' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
         // WITHOUT JOKER
   //    match card {
   //      '2' => 1,
   //      '3' => 2,
   //      '4' => 3,
   //      '5' => 4,
   //      '6' => 5,
   //      '7' => 6,
   //      '8' => 7,
   //      '9' => 8,
   //      'T' => 9,
   //      'J' => 10,
   //      'Q' => 11,
   //      'K' => 12,
   //      'A' => 13,
   //      _ => 0,
   //  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    High,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
   hand: Vec<char>,
   hand_type: HandType,
   bid: u32,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
               // when hand types are equal
               // compare cards letter by letter

               for (self_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
                  let self_strength = get_card_strength(self_card);
                  let other_strength = get_card_strength(other_card);
                  let cmp_result = self_strength.cmp(&other_strength);

                  if cmp_result != std::cmp::Ordering::Equal {
                     return cmp_result
                  }
               }

               std::cmp::Ordering::Equal
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Card {
   fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       Some(self.cmp(other))
   }
}

fn get_hand_type(hand: &Vec<char>, with_joker: bool) -> HandType {
   let mut symbols: HashMap<char, u32> = HashMap::new();
   hand.iter().for_each(|&card| {
      symbols.entry(card).and_modify(|c| *c += 1).or_insert(1);
   });

   if with_joker {
      // get the J count
      let joker_count = symbols.get(&'J').unwrap_or(&0).clone();
      println!("jokers: {joker_count}");

      if joker_count > 0 && joker_count < 5 {
         // clear out the J
         symbols.remove(&'J');

         // find what is the most common card in remaining deck
         let most_common = symbols.iter().max_by_key(|entry| entry.1).unwrap().0;
         println!("Most common symbol for deck {:?} is {most_common}", &hand);

         symbols.entry(*most_common).and_modify(|count| *count += joker_count);
      }
   }

   match symbols.len() {
      1 => HandType::FiveOfAKind,
      2 => {
         let values: Vec<u32> = symbols.values().cloned().collect();
         if values.contains(&4) { HandType::FourOfAKind } else { HandType::FullHouse }
      },
      3 => {
         let values: Vec<u32> = symbols.values().cloned().collect();
         if values.contains(&3) { HandType::ThreeOfAKind } else { HandType::TwoPair }
      },
      4 => HandType::OnePair,
      5 => HandType::High,
      _ => HandType::High,
   }
}

fn parse_lines(lines: Vec<String>, with_joker: bool) -> Vec<Card> {
   lines.iter().map(|line| {
      let (hand_str, bid_str) = line.split_once(" ").unwrap_or_default();
      let hand: Vec<char> = hand_str.chars().collect();
      let bid = bid_str.parse::<u32>().unwrap_or_default();

      let hand_type = get_hand_type(&hand, with_joker);
      dbg!(&hand_type);

      Card { hand, bid, hand_type }
   }).collect()
}

pub fn task1() {
    let lines = read_input_file("y2023", "day7");
    let mut parsed_lines = parse_lines(lines, false);
    parsed_lines.sort();

    let total = parsed_lines.iter().enumerate().fold(0, |sum, (index, card)| {
         sum + (index as u32 + 1) * card.bid
    });

    dbg!(total);
}

pub fn task2() {
   let lines = read_input_file("y2023", "day7");
    let mut parsed_lines = parse_lines(lines, true);
    parsed_lines.sort();

    let total = parsed_lines.iter().enumerate().fold(0, |sum, (index, card)| {
         sum + (index as u32 + 1) * card.bid
    });

    dbg!(total);
} 
