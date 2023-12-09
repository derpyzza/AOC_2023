use std::fs;

#[derive(Debug)]
enum HandType { 
    FIVEOFKIND,
    FOUROFKIND,
    FULLHOUSE,
    THREEOFKIND,
    TWOPAIR,
    ONEPAIR,
    HIGHCARD,
    UNREACHABLE,
}

#[derive(Debug)]
struct Card {
    _hand_type: HandType, // this and _data were put in mostly just for debug printing.
    _data: [char; 5],
    strength: u64,
    bid: u32,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.strength.partial_cmp(&other.strength)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.strength.cmp(&other.strength)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Eq for Card {}

fn determine_type(hand: [char; 5], joker_wildcard: bool) -> HandType {

    let mut cards: [u8; 13] = [0; 13];
    let mut joker: u8 = 0;

    for card in hand.iter() {
        match card {
            'A' => cards[00] += 1,
            'K' => cards[01] += 1,
            'Q' => cards[02] += 1,
            'T' => cards[03] += 1,
            '9' => cards[04] += 1,
            '8' => cards[05] += 1,
            '7' => cards[06] += 1,
            '6' => cards[07] += 1,
            '5' => cards[08] += 1,
            '4' => cards[09] += 1,
            '3' => cards[10] += 1,
            '2' => cards[11] += 1,
            'J' => { if joker_wildcard { joker += 1} else { cards[12] += 1 } },
            _   => {}
        }
    }

    if joker_wildcard {
        // println!("card: {:#?}", hand);
        let mut largest = 0;
        let mut _id = 0;
        for (id, c) in cards.into_iter().enumerate() {
            if id != 12 {
                if c > largest {
                    // println!("largest: {}, c: {:#?}", largest, c);
                    largest = c;
                    _id = id;
                }
            }
        }
        cards[_id] += joker;
    }
    

    let mut non_zero: Vec<u8> = Vec::new();
    for c in cards {
        if c != 0 { 
            non_zero.push(c) 
        }
    }

    non_zero.sort();

    match non_zero[..] {
        [1, 1, 1, 1, 1] => HandType::HIGHCARD,
        [1, 1, 1, 2]    => HandType::ONEPAIR,
        [1, 2, 2]       => HandType::TWOPAIR,
        [1, 1, 3]       => HandType::THREEOFKIND,
        [2, 3]          => HandType::FULLHOUSE,
        [1, 4]          => HandType::FOUROFKIND,
        [5]             => HandType::FIVEOFKIND,
        _               => HandType::UNREACHABLE, // this shouldn't happen;
    }
}

fn return_strength(chars: [char; 5], prefix: u32, joker_wildcard: bool) -> u64 {
    let mut str = prefix.to_string();

    for char in chars {
        let x = match char {
            'A' => 24,
            'K' => 23,
            'Q' => 22,
            'J' => { if joker_wildcard { 11 } else { 21 } },
            'T' => 20,
            '9' => 19,
            '8' => 18,
            '7' => 17,
            '6' => 16,
            '5' => 15,
            '4' => 14,
            '3' => 13,
            '2' => 12,
            val => panic!("Shouldn't happen: {}", val),
        };

        str += x.to_string().as_str();
    }
    str.parse::<u64>().unwrap()
}

fn get_hand_strength (hand: &HandType) -> u32 {
    match hand {
        HandType::FIVEOFKIND    => 7,
        HandType::FOUROFKIND    => 6,
        HandType::FULLHOUSE     => 5,
        HandType::THREEOFKIND   => 4,
        HandType::TWOPAIR       => 3,
        HandType::ONEPAIR       => 2,
        HandType::HIGHCARD      => 1,
        _   => 0,
    }
}

/**
 * Returns sorted list of cards
 */
fn return_cards_list(contents: String, joker_wildcard: bool) -> Vec<Card> {

    // Split lines into the Card struct
    // This first line splits every line in the input into it's own string 
    let mut cards: Vec<Card> = contents.split("\n")
        .map(|line| {
            
            let split: Vec<&str> = line.split_whitespace().collect();
            if split.is_empty() {
                None
            } else {

                // The card data ( KKQ12 ) is extracted as a list of chars
                let raw_data = split[0].chars();
                // the bid
                let _bid = split[1].parse::<u32>().unwrap();

                // the chars are then arranged in an array 
                let data: [char; 5] = raw_data
                    .into_iter()
                    .map(|x| { 
                        x
                    })
                    .collect::<Vec<_>>().try_into().expect("Error creating card data");

                let hand_type = determine_type(data, joker_wildcard);

                // Assigns a strength value to the cards, which is then used to sort the cards;
                let str = return_strength(data, get_hand_strength(&hand_type), joker_wildcard);

                let card: Card = Card {
                    strength: str,
                    _hand_type: hand_type,
                    _data: data,
                    bid: _bid,
                };

                Some(card)
            }
        }).filter_map(|x| x).collect();

    cards.sort_by(|x, y| x.cmp(y));

    cards
}

fn solve_part_one(contents: String) -> u32 {
    // don't consider jokers as wildcards
    let cards = return_cards_list(contents, false);
    let mut total = 0;
    for (id, card) in cards.into_iter().enumerate() {
        total += card.bid * ((id + 1) as u32);
    }
    total
}

fn solve_part_two(contents: String) -> u32 {
    // consider jokers as wildcards
    let cards = return_cards_list(contents, true);
    let mut total = 0;
    for (id, card) in cards.into_iter().enumerate() {
        total += card.bid * ((id + 1) as u32);
    }
    total
}

fn main() {

    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let first = solve_part_one(contents.clone());
    let second = solve_part_two(contents.clone());
    println!("first: {}", first);
    println!("second: {}", second);
}
