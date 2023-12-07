use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

pub fn main() -> String {
    let dir = std::env::current_dir().unwrap();

    let mut file = File::open(format!("{}/day07/src/in.txt", dir.display())).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    format!("Total winning: {}", solution(&contents))
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn compute_hand_type(hand: &str) -> HandType {
    let j_count = hand.matches('J').count();

    let mut hand = hand.chars().collect::<Vec<char>>();
    hand.sort();
    let mut deduped_hand = hand.into_iter().dedup_with_count().collect_vec();

    deduped_hand.sort_by(|a, b| a.0.cmp(&b.0));

    let hand_type = match deduped_hand.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if j_count >= 1 {
                HandType::FiveOfAKind
            } else {
                if deduped_hand[1/*len - 1 */].0 == 4 {
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
        }
        3 => {
            if deduped_hand[2 /*len -1 */].0 == 3 {
                if j_count >= 1 {
                    HandType::FourOfAKind
                } else {
                    HandType::ThreeOfAKind
                }
            } else {
                if j_count == 1 {
                    HandType::FullHouse
                } else if j_count == 2 {
                    HandType::FourOfAKind
                } else {
                    HandType::TwoPairs
                }
            }
        }
        4 => {
            if j_count == 1 || j_count == 2 {
                HandType::ThreeOfAKind
            } else {
                HandType::OnePair
            }
        }
        _ => {
            if j_count == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    };

    hand_type
}

fn extract_hand_and_its_bid(input: &str) -> (&str, u32) {
    let mut part = input.split(" ");
    let hand = part.next().unwrap();
    let bid = part.next().unwrap().parse::<u32>().unwrap();

    (hand, bid)
}

fn translate_card_label_to_number(card: char) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => card.to_digit(10).unwrap(),
    }
}

fn compare_two_hands_by_its_cards(a: &str, b: &str) -> Ordering {
    let mut first_card = a.chars();
    let mut second_card = b.chars();

    while let Some(a) = first_card.next() {
        let a = translate_card_label_to_number(a);
        let b = second_card.next().unwrap();
        let b = translate_card_label_to_number(b);

        if a > b {
            return Ordering::Greater;
        } else if a < b {
            return Ordering::Less;
        }
    }

    Ordering::Equal
}

fn compare_two_hands_by_their_type(a: &HandType, b: &HandType) -> Ordering {
    if a > b {
        return Ordering::Greater;
    } else if a < b {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn compare_two_hands(a: Hand, b: Hand) -> Ordering {
    let a_compared_with_b_by_type = compare_two_hands_by_their_type(&a.hand_type, &b.hand_type);

    if a_compared_with_b_by_type != Ordering::Equal {
        return a_compared_with_b_by_type;
    } else {
        return compare_two_hands_by_its_cards(a.hand, b.hand);
    }
}

#[derive(Clone, Copy, Debug)]
struct Hand<'a> {
    hand: &'a str,
    bid: u32,
    hand_type: HandType,
}

fn solution(input: &str) -> u64 {
    let mut list_of_hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let hand_and_its_bid = extract_hand_and_its_bid(line);
            let hand_type = compute_hand_type(hand_and_its_bid.0);
            Hand {
                hand: hand_and_its_bid.0,
                bid: hand_and_its_bid.1,
                hand_type,
            }
        })
        .collect();

    let hand = "T55J5";
    compute_hand_type(hand);

    list_of_hands.sort_by(|a, b| compare_two_hands(*a, *b));

    list_of_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1)) as u64
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_can_extract_hand_and_its_bid() {
        let input = "AAAAA 100";
        assert_eq!(super::extract_hand_and_its_bid(input), ("AAAAA", 100));

        let input = "AA8AA 1";
        assert_eq!(super::extract_hand_and_its_bid(input), ("AA8AA", 1));
    }

    #[test]
    fn it_can_compute_hand_type() {
        let hands = vec!["32T3K", "T55J5", "KK677", "KTJJT", "QQQJA", "A3JJ4"];
        let expected = vec![
            super::HandType::OnePair,
            super::HandType::FourOfAKind,
            super::HandType::TwoPairs,
            super::HandType::FourOfAKind,
            super::HandType::FourOfAKind,
            super::HandType::ThreeOfAKind,
        ];
        for (hand, expected) in hands.iter().zip(expected.iter()) {
            assert_eq!(super::compute_hand_type(hand), *expected);
        }
    }

    #[test]
    fn it_can_translate_card_label_to_number() {
        let cards = vec![
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
        ];
        let expected = vec![14, 13, 12, 1, 10, 9, 8, 7, 6, 5, 4, 3, 2];
        for (card, expected) in cards.iter().zip(expected.iter()) {
            assert_eq!(super::translate_card_label_to_number(*card), *expected);
        }
    }

    #[test]
    fn it_can_compare_two_hands_by_cards() {
        assert_eq!(
            super::compare_two_hands_by_its_cards("AAAAA", "AAAAA"),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            super::compare_two_hands_by_its_cards("AAAAA", "AA8AA"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_its_cards("AA8AA", "23332"),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_its_cards("23332", "TTT98"),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::compare_two_hands_by_its_cards("TT298", "TT432"),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            super::compare_two_hands_by_its_cards("TTJ98", "TT432"),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn it_can_compare_two_hands_by_their_type() {
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::FiveOfAKind,
                &super::HandType::FiveOfAKind
            ),
            std::cmp::Ordering::Equal
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::FiveOfAKind,
                &super::HandType::FourOfAKind
            ),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::FourOfAKind,
                &super::HandType::FullHouse
            ),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::FullHouse,
                &super::HandType::ThreeOfAKind
            ),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::ThreeOfAKind,
                &super::HandType::TwoPairs
            ),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::TwoPairs,
                &super::HandType::OnePair
            ),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            super::compare_two_hands_by_their_type(
                &super::HandType::OnePair,
                &super::HandType::HighCard
            ),
            std::cmp::Ordering::Greater
        );
    }
}
