use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum CardType {
    StraightFlush = 8,
    FourOfAKind = 7,
    FullHouse = 6,
    Flush = 5,
    Straight = 4,
    ThreeOfAKind = 3,
    TwoPairs = 2,
    OnePairs = 1,
    HighCard = 0,
}

#[derive(Eq, Debug)]
struct Hand {
    card_type: CardType,
    cards: String,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.card_type.cmp(&other.card_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.card_type == other.card_type
    }
}

impl Hand {
    fn new(cards: &str) -> Hand {
        // let mut _cards = cards.split(' ').collect::<Vec<_>>();
        // let _cards = _cards
        //     .iter()
        //     .map(|c| c.to_string())
        //     .collect::<Vec<String>>();
        let _cards = cards.to_string();
        if is_straight_flush(cards) {
            Hand {
                card_type: CardType::StraightFlush,
                cards: _cards,
            }
        } else if is_four_of_a_hands(cards) {
            Hand {
                card_type: CardType::FourOfAKind,
                cards: _cards,
            }
        } else if is_full_house(cards) {
            Hand {
                card_type: CardType::FullHouse,
                cards: _cards,
            }
        } else if is_flush(cards) {
            Hand {
                card_type: CardType::Flush,
                cards: _cards,
            }
        } else if is_straight(cards) {
            Hand {
                card_type: CardType::Straight,
                cards: _cards,
            }
        } else if is_three_of_a_kind(cards) {
            Hand {
                card_type: CardType::ThreeOfAKind,
                cards: _cards,
            }
        } else if is_two_pairs(cards) {
            Hand {
                card_type: CardType::TwoPairs,
                cards: _cards,
            }
        } else if is_one_pairs(cards) {
            Hand {
                card_type: CardType::OnePairs,
                cards: _cards,
            }
        } else {
            Hand {
                card_type: CardType::HighCard,
                cards: _cards,
            }
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

fn is_all_same<T: PartialEq>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] == w[1])
}

fn identify_card_type(hand: &str) -> CardType {
    if is_straight_flush(hand) {
        CardType::StraightFlush
    } else if is_four_of_a_hands(hand) {
        CardType::FourOfAKind
    } else if is_full_house(hand) {
        CardType::FullHouse
    } else if is_flush(hand) {
        CardType::Flush
    } else if is_straight(hand) {
        CardType::Straight
    } else if is_three_of_a_kind(hand) {
        CardType::ThreeOfAKind
    } else if is_two_pairs(hand) {
        CardType::TwoPairs
    } else if is_one_pairs(hand) {
        CardType::OnePairs
    } else {
        CardType::HighCard
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // 比牌型，如果都大於別人則直接勝出
    // 先做牌型歸類，將每種牌型都先分門別類，然後找出最大的牌型，在進行進一步篩選
    let mut card_type_separate = HashMap::new();
    let mut card_type: Vec<CardType> = Vec::new();
    for (index, h) in hands.iter().enumerate() {
        let current_type = identify_card_type(h);
        card_type_separate
            .entry(current_type)
            .or_insert_with(Vec::new)
            .push(index);
    }
    let mut biggest = CardType::HighCard;
    // 看哪個牌型最大，把最大的牌型裡面的元素抓出來比較
    for k in card_type_separate.keys() {
        if *k > biggest {
            biggest = (*k).clone();
        }
    }
    let biggest_card_type = card_type_separate.get(&biggest).unwrap();
    // 如果最大牌型只有一個元素，可以直接回傳
    // 若有多個，比數字
    if biggest_card_type.len() == 1 {
        return vec![hands[biggest_card_type[0]]];
    } else {
        if biggest == CardType::StraightFlush {
            find_biggest_face();
        } else if biggest == CardType::FourOfAKind {
            find_biggest_face();
        } else if biggest == CardType::FullHouse {
            find_biggest_face();
        } else if biggest == CardType::Flush {
            find_biggest_face();
        } else if biggest == CardType::Straight {
            find_biggest_face();
        } else if biggest == CardType::ThreeOfAKind {
            find_biggest_face();
        } else if biggest == CardType::TwoPairs {
            find_biggest_face();
        } else if biggest == CardType::OnePairs {
            find_biggest_face();
        } else {
            find_biggest_face();
        }
        for condidate in biggest_card_type.iter() {}
        let biggest_face = match biggest_card_type {
            CardType::StraightFlush => 1,
            _ => 2,
        };
    }
    card_type.sort();

    let mut vec_hands: Vec<Hand> = Vec::new();
    for h in hands {
        vec_hands.push(Hand::new(h));
    }
    vec_hands.sort();
    println!("{:?}", vec_hands);
    if vec_hands.len() == 1 {
        return hands.to_vec();
    }
    if is_all_same(hands) {
        return hands.to_vec();
    }
    // let mut scores: Vec<(usize, i32)> = Vec::new();
    // if hands.len() == 1 {
    //     return hands.to_vec();
    // }
    // for (idx, hand) in hands.iter().enumerate() {
    //     if is_straight_flush(hand) {
    //         scores.push((idx, 9));
    //     } else if is_four_of_a_hands(hand) {
    //         scores.push((idx, 8));
    //     } else if is_full_house(hand) {
    //         scores.push((idx, 7));
    //     } else if is_flush(hand) {
    //         scores.push((idx, 6));
    //     } else if is_straight(hand) {
    //         scores.push((idx, 5));
    //     } else if is_three_of_a_kind(hand) {
    //         scores.push((idx, 4));
    //     } else if is_two_pairs(hand) {
    //         scores.push((idx, 3));
    //     } else if is_one_pairs(hand) {
    //         scores.push((idx, 2));
    //     } else if is_high_card(hand) {
    //         scores.push((idx, 1));
    //     }
    // }

    unimplemented!("Out of {hands:?}, which hand wins?")
}

fn is_high_card(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut card_numbers = Vec::new();
    for card in cards.iter() {
        let mut card_chars = card.chars();
        let card_number = card_chars.next().unwrap();
        match card_number {
            'J' => card_numbers.push("11".to_owned()),
            'Q' => card_numbers.push("12".to_owned()),
            'K' => card_numbers.push("13".to_owned()),
            _ => card_numbers.push(card_number.to_string()),
        }
    }
    let mut card_numbers = card_numbers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    card_numbers.sort_unstable();
    let max_number = card_numbers.iter().max().unwrap();
    true
}

fn is_one_pairs(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut count = HashMap::new();
    for card in cards.iter() {
        let card_number = card.chars().next().unwrap();
        *count.entry(card_number).or_insert(0) += 1;
    }
    let mut pairs_times = 0;
    for k in count.keys() {
        if count.get(k) == Some(&2) {
            pairs_times += 1;
        }
    }
    pairs_times == 1
}

fn is_two_pairs(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut count = HashMap::new();
    for card in cards.iter() {
        let card_number = card.chars().next().unwrap();
        *count.entry(card_number).or_insert(0) += 1;
    }
    let mut pairs_times = 0;
    for k in count.keys() {
        if count.get(k) == Some(&2) {
            pairs_times += 1;
        }
    }
    pairs_times == 2
}

fn is_three_of_a_kind(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut count = HashMap::new();
    for card in cards.iter() {
        let card_number = card.chars().next().unwrap();
        *count.entry(card_number).or_insert(0) += 1;
    }
    for k in count.keys() {
        if count.get(k) == Some(&3) {
            return true;
        }
    }
    false
}

fn is_straight(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut card_numbers = Vec::new();
    for card in cards.iter() {
        let mut card_chars = card.chars();
        let card_number = card_chars.next().unwrap();
        match card_number {
            'J' => card_numbers.push("11".to_owned()),
            'Q' => card_numbers.push("12".to_owned()),
            'K' => card_numbers.push("13".to_owned()),
            _ => card_numbers.push(card_number.to_string()),
        }
    }
    let mut card_numbers = card_numbers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    card_numbers.sort_unstable();
    let min_numbers = card_numbers.iter().min().unwrap();
    let expected_straight = (*min_numbers..(min_numbers + 4)).collect::<Vec<i32>>();
    card_numbers == expected_straight
}

fn is_flush(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut cards_suit: HashSet<String> = HashSet::new();
    for card in cards.iter() {
        let card_suit = card.chars().nth(1).unwrap();
        cards_suit.insert(card_suit.to_string());
    }
    cards_suit.len() == 1
}

fn is_straight_flush(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut cards_suit: HashSet<String> = HashSet::new();
    let mut card_numbers = Vec::new();
    for card in cards.iter() {
        let mut card_chars = card.chars();
        let card_number = card_chars.next().unwrap();
        match card_number {
            'J' => card_numbers.push("11".to_owned()),
            'Q' => card_numbers.push("12".to_owned()),
            'K' => card_numbers.push("13".to_owned()),
            _ => card_numbers.push(card_number.to_string()),
        }
        let card_suit = card_chars.next().unwrap();
        cards_suit.insert(card_suit.to_string());
    }
    let mut card_numbers = card_numbers
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    card_numbers.sort_unstable();
    let min_numbers = card_numbers.iter().min().unwrap();
    let expected_straight = (*min_numbers..(min_numbers + 4)).collect::<Vec<i32>>();
    card_numbers == expected_straight && cards_suit.len() == 1
}

fn is_four_of_a_hands(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut count = HashMap::new();
    for card in cards.iter() {
        let card_number = card.chars().next().unwrap();
        *count.entry(card_number).or_insert(0) += 1;
    }
    for k in count.keys() {
        if count.get(k) == Some(&4) {
            return true;
        }
    }
    false
}

fn is_full_house(hand: &str) -> bool {
    let cards = hand.split(' ').collect::<Vec<_>>();
    let mut count = HashMap::new();
    let mut full = false;
    let mut pair = false;
    for card in cards.iter() {
        let card_number = card.chars().next().unwrap();
        *count.entry(card_number).or_insert(0) += 1;
    }
    for k in count.keys() {
        if count.get(k) == Some(&3) {
            full = true;
        } else if count.get(k) == Some(&2) {
            pair = true;
        }
    }

    full && pair
}
