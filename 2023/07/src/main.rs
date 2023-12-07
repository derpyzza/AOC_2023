use std::fs;

// A K Q J T 9 8 7 6 5 4 3 2
//


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
    hand_type: HandType,
    data: [char; 5],
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

fn determine_type(hand: [char; 5]) -> HandType {

    let mut cards: [u8; 13] = [0; 13];

    for card in hand.iter() {
        match card {
            'A' => cards[00] += 1,
            'K' => cards[01] += 1,
            'Q' => cards[02] += 1,
            'J' => cards[03] += 1,
            'T' => cards[04] += 1,
            '9' => cards[05] += 1,
            '8' => cards[06] += 1,
            '7' => cards[07] += 1,
            '6' => cards[08] += 1,
            '5' => cards[09] += 1,
            '4' => cards[10] += 1,
            '3' => cards[11] += 1,
            '2' => cards[12] += 1,
            _   => {}
        }
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

fn return_strength(chars: [char; 5], prefix: u32) -> u64 {
    let mut str = prefix.to_string();

    for char in chars {
        let x = match char {
            'A' => 24,
            'K' => 23,
            'Q' => 22,
            'J' => 21,
            'T' => 20,
            '9' => 19,
            '8' => 18,
            '7' => 17,
            '6' => 16,
            '5' => 15,
            '4' => 14,
            '3' => 13,
            '2' => 12,
            _   => 0,
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

fn main() {

    let file_path = "./input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut cards: Vec<Card> = contents.split("\n")
        .map(|line| {
            let split: Vec<&str> = line.split_whitespace().collect();
            if split.is_empty() {
                None
            } else {
                let raw_data = split[0].chars();
                let bid = split[1].parse::<u32>().unwrap();

                let data: [char; 5] = raw_data
                    .into_iter()
                    .map(|x| { 
                        x
                    })
                    .collect::<Vec<_>>().try_into().expect("Error creating card data");

                let hand_type = determine_type(data);

                let mut str = return_strength(data, get_hand_strength(&hand_type));

                let card: Card = Card {
                    strength: str,
                    hand_type: hand_type,
                    data: data,
                    bid: bid,
                };

                Some(card)
            }
        }).filter_map(|x| x).collect();

    cards.sort_by(|x, y| x.cmp(y));

    let mut total = 0;
    for (id, card) in cards.into_iter().enumerate() {
        println!("data: {:#?}, str: {}", card.data, card.strength);
        println!("id: {id}, bid: {}", card.bid);
        println!("type: {:#?}", card.hand_type);
        total += (card.bid * ((id + 1) as u32));
    }
    println!("{}", total);
}
