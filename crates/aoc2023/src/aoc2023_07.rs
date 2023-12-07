use aoclib::Runner;

#[derive(Default)]
pub struct Aoc2023_07 {
    hands: Vec<Hand>,
}

impl Aoc2023_07 {
    pub fn new() -> Self {
        Self::default()
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
            // println!("{:?} score = {:x}", hand.hand, hand.score);
            self.hands.push(hand);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        self.hands.sort_by(|h1, h2| h1.score.cmp(&h2.score));
        /*
        for (idx, hand) in self.hands.iter().enumerate() {
            println!("{:}: {hand:?} -> {}", (idx + 1), (idx + 1) * hand.bet);
        }
        */
        aoclib::output(
            self.hands
                .iter()
                .enumerate()
                .fold(0, |acc, (index, hand)| acc + (index + 1) * hand.bet),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct Hand {
    _hand: Vec<Card>,
    score: usize,
    bet: usize,
}

impl Hand {
    fn new(hand: &[Card], bet: usize) -> Self {
        let score = Self::score(hand);
        Self {
            _hand: hand.to_vec(),
            score,
            bet,
        }
    }

    fn score(hand: &[Card]) -> usize {
        let mut dup: Vec<Card> = hand.into();

        dup.sort();

        let ht = if dup[0] == dup[4] {
            HandType::FiveOfAKind
        } else if dup[0] == dup[3] || dup[1] == dup[4] {
            HandType::FourOfAKind
        } else if (dup[0] == dup[1] && dup[2] == dup[4]) || (dup[0] == dup[2] && dup[3] == dup[4]) {
            HandType::FullHouse
        } else if dup[0] == dup[2] || dup[1] == dup[3] || dup[2] == dup[4] {
            HandType::ThreeOfAKind
        } else {
            let pair_count = dup
                .as_slice()
                .windows(2)
                .filter(|pair| pair[0] == pair[1])
                .count();
            match pair_count {
                0 => HandType::HighCard,
                1 => HandType::OnePair,
                2 => HandType::TwoPair,
                _ => panic!("code bug"),
            }
        };

        let mut score = ht as usize;
        for c in hand {
            score = (score << 4) | (*c as usize);
        }

        score
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
    Two, // 0x0
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
    Ace, // 0xc
}

impl Card {
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
