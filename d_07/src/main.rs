use itertools::Itertools;
use std::{collections::HashMap, iter::Sum};

#[derive(PartialEq, Debug)]
enum Hands {
    FiveOfAKind(usize),
    FourOfAKind(usize),
    FullHouse(usize),
    ThreeOfAKind(usize),
    TwoPair(usize),
    OnePair(usize),
    HighCard(usize),
    None,
}

impl Hands {
    pub fn to_int(&self) -> Option<usize> {
        match self {
            Hands::FiveOfAKind(x)
            | Hands::FourOfAKind(x)
            | Hands::FullHouse(x)
            | Hands::ThreeOfAKind(x)
            | Hands::TwoPair(x)
            | Hands::OnePair(x)
            | Hands::HighCard(x) => Some(*x),
            Hands::None => None,
        }
    }
}

impl Copy for Hands {}
impl Clone for Hands {
    fn clone(&self) -> Hands {
        *self
    }
}

fn convert_hand_to_value(hand: &str) -> (Hands, Vec<usize>) {
    let card_values: HashMap<char, usize> = HashMap::from([
        ('2', 0),
        ('3', 1),
        ('4', 2),
        ('5', 3),
        ('6', 4),
        ('7', 5),
        ('8', 6),
        ('9', 7),
        ('T', 8),
        ('J', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
    let mut card_count: [usize; 13] = [0; 13];

    let hand_with_values: Vec<usize> = hand
        .chars()
        .map(|card| *card_values.get_key_value(&card).unwrap().1)
        .collect();
    for value in &hand_with_values {
        card_count[*value] += 1;
    }

    let collected_hands: Vec<_> = card_count
        .map(|card| {
            let mut hand_val: Hands = Hands::None;
            if card == 5 {
                hand_val = Hands::FiveOfAKind(6);
            }
            if card == 4 {
                hand_val = Hands::FourOfAKind(5);
            }
            if card == 3 {
                hand_val = Hands::ThreeOfAKind(3);
            }
            if card == 2 {
                hand_val = Hands::OnePair(1);
            }
            if card == 1 {
                hand_val = Hands::HighCard(0);
            }
            hand_val
        })
        .into_iter()
        .filter(|hand_type| *hand_type != Hands::None)
        .collect();
    let mut best_hand = {
        let mut bh = Hands::None;
        if collected_hands.contains(&Hands::HighCard(0)) {
            bh = Hands::HighCard(0);
        }
        if collected_hands.contains(&Hands::OnePair(1)) {
            bh = Hands::OnePair(1);
        }
        if collected_hands.contains(&Hands::ThreeOfAKind(3)) {
            bh = Hands::ThreeOfAKind(3);
        }
        if collected_hands.contains(&Hands::FourOfAKind(5)) {
            bh = Hands::FourOfAKind(5);
        }
        if collected_hands.contains(&Hands::FiveOfAKind(6)) {
            bh = Hands::FiveOfAKind(6);
        }
        bh
    };
    if collected_hands
        .iter()
        .filter(|hand| **hand == Hands::OnePair(1))
        .count()
        == 2
    {
        best_hand = Hands::TwoPair(2);
    }
    if collected_hands.contains(&Hands::OnePair(1))
        && collected_hands.contains(&Hands::ThreeOfAKind(3))
    {
        best_hand = Hands::FullHouse(4);
    }
    (best_hand, hand_with_values)
}

fn sort_hands(unsorted_hands: &[(Hands, Vec<usize>, usize)]) -> Vec<(Hands, Vec<usize>, usize)> {
    let mut five_of_a_kinds: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut four_of_a_kinds: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut full_houses: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut three_of_a_kinds: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut two_pairs: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut pairs: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    let mut high_hards: Vec<&(Hands, Vec<usize>, usize)> = vec![];
    for hand in unsorted_hands {
        match hand.0 {
            Hands::FiveOfAKind(x) => five_of_a_kinds.push(hand),
            Hands::FourOfAKind(x) => four_of_a_kinds.push(hand),
            Hands::FullHouse(x) => full_houses.push(hand),
            Hands::ThreeOfAKind(x) => three_of_a_kinds.push(hand),
            Hands::TwoPair(x) => two_pairs.push(hand),
            Hands::OnePair(x) => pairs.push(hand),
            Hands::HighCard(x) => high_hards.push(hand),
            Hands::None => (),
        }
    }
    let mut sorted_hands: Vec<(Hands, Vec<usize>, usize)> = vec![];
    sorted_hands
}

fn main() {
    let data_input: Vec<(&str, usize)> = include_str!("example.txt")
        .trim_end()
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_whitespace().collect_tuple().unwrap();
            (hand, bet.parse::<usize>().unwrap())
        })
        .collect();

    //println!("{:#?}", data_input);
    println!("{:#?}", convert_hand_to_value(data_input[2].0));
    let converted_data: Vec<(Hands, Vec<usize>, usize)> = data_input
        .iter()
        .map(|hand| {
            let convert_hand: (Hands, Vec<usize>) = convert_hand_to_value(hand.0);
            (convert_hand.0, convert_hand.1, hand.1)
        })
        .collect();
    println!("{:#?}", converted_data);
}
