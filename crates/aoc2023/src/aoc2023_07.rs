use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_07 {
    hands: Vec<Hand>,
}

impl Aoc2023_07 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn score_hands(&self) -> usize {
        self.hands
            .iter()
            .enumerate()
            .fold(0, |acc, (index, hand)| acc + (index + 1) * hand.bet)
    }
}

impl Runner for Aoc2023_07 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse(&mut self) {
        let _lines = [
            "32T3K 765".to_string(),
            "T55J5 684".to_string(),
            "KK677 28".to_string(),
            "KTJJT 220".to_string(),
            "QQQJA 483".to_string(),
        ];
        let lines = aoclib::read_lines("input/2023-07.txt");

        for line in &lines {
            let (hand, bet) = line.split_once(' ').unwrap();
            let bet = bet.parse().unwrap();

            let hand = hand.chars().map(Card::from_char).collect::<Vec<_>>();
            let hand = Hand::new(&hand, bet);

            self.hands.push(hand);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.hands.sort_by(|h1, h2| h1.score.cmp(&h2.score));
        aoclib::output(self.score_hands())
    }

    fn part2(&mut self) -> Vec<String> {
        self.hands.sort_by(|h1, h2| h1.wc_score.cmp(&h2.wc_score));
        aoclib::output(self.score_hands())
    }
}

#[derive(Debug)]
struct Hand {
    _hand: Vec<Card>,
    score: usize,
    wc_score: usize,
    bet: usize,
}

impl Hand {
    fn new(hand: &[Card], bet: usize) -> Self {
        let score = Self::score(hand);
        let wc_score = Self::wc_score(hand);
        Self {
            _hand: hand.to_vec(),
            wc_score,
            score,
            bet,
        }
    }

    fn score(hand: &[Card]) -> usize {
        let mut dup: Vec<Card> = hand.into();

        dup.sort();
        let jcount = dup.iter().filter(|card| **card == Card::Joker).count();

        let ht = if dup[0].is(dup[4]) {
            HandType::FiveOfAKind
        } else if dup[0].is(dup[3]) || dup[1].is(dup[4]) {
            if jcount == 1 {
                HandType::FiveOfAKind
            } else {
                assert!(jcount == 0);
                HandType::FourOfAKind
            }
        } else if (dup[0].is(dup[1]) && dup[2].is(dup[4]))
            || (dup[0].is(dup[2]) && dup[3].is(dup[4]))
        {
            HandType::FullHouse
        } else if dup[0].is(dup[2]) || dup[1].is(dup[3]) || dup[2].is(dup[4]) {
            if jcount == 0 {
                HandType::ThreeOfAKind
            } else if jcount == 1 {
                HandType::FourOfAKind
            } else if jcount == 2 {
                HandType::FiveOfAKind
            } else {
                panic!("code bug");
            }
        } else {
            let pair_count = dup
                .as_slice()
                .windows(2)
                .filter(|pair| pair[0].is(pair[1]))
                .count();

            if jcount == 0 {
                match pair_count {
                    0 => HandType::HighCard,
                    1 => HandType::OnePair,
                    2 => HandType::TwoPair,
                    _ => panic!("code bug"),
                }
            } else if jcount == 1 {
                match pair_count {
                    0 => HandType::OnePair,
                    1 => HandType::ThreeOfAKind,
                    2 => HandType::FullHouse,
                    _ => panic!("code bug"),
                }
            } else if jcount == 2 {
                match pair_count {
                    0 => HandType::ThreeOfAKind,
                    1 => HandType::FourOfAKind,
                    _ => panic!("code bug: {pair_count}: {hand:?}"),
                }
            } else if jcount == 3 && pair_count == 1 || jcount >= 4 {
                HandType::FiveOfAKind
            } else if jcount == 3 {
                HandType::FourOfAKind
            } else {
                HandType::HighCard
            }
        };

        let mut score = ht as usize;
        for c in hand {
            score = (score << 4) | (*c as usize);
        }

        score
    }

    fn wc_score(hand: &[Card]) -> usize {
        let wc_hand: Vec<Card> = hand
            .iter()
            .map(|card| {
                if *card == Card::Jack {
                    Card::Joker
                } else {
                    *card
                }
            })
            .collect();

        Self::score(&wc_hand)
    }
}

// 0x100123

#[derive(Debug)]
enum HandType {
    HighCard, // 0x0
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind, // 0x6
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace, // 0xd
}

impl Card {
    fn is(self, other: Card) -> bool {
        if self == Self::Joker || other == Self::Joker {
            false
        } else {
            self == other
        }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            '2' => Self::Two, // 0x0
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("code bug"),
        }
    }
}
