use anyhow::Context;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(char: char) -> anyhow::Result<Self> {
        let card = match char {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => anyhow::bail!("Unknown card char '{char}'"),
        };
        Ok(card)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn find(cards: &[Card; 5]) -> HandType {
        let mut stuff = [0; 13];
        let mut jokers = 0;
        for card in cards {
            if *card == Card::J {
                jokers += 1;
            } else {
                stuff[*card as usize] += 1;
            }
        }
        stuff.sort_by(|a, b| b.cmp(a));
        stuff[0] += jokers;

        if stuff[0] == 5 {
            return HandType::FiveOfAKind;
        }

        if stuff[0] == 4 {
            return HandType::FourOfAKind;
        }

        if stuff[0] == 3 && stuff[1] == 2 {
            return HandType::FullHouse;
        }

        if stuff[0] == 3 && stuff[1] == 1 {
            return HandType::ThreeOfAKind;
        }

        if stuff[0] == 2 && stuff[1] == 2 {
            return HandType::TwoPair;
        }

        if stuff[0] == 2 && stuff[1] == 1 && stuff[2] == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
}

impl Hand {
    fn parse(input: &str) -> anyhow::Result<Self> {
        let cards: [Card; 5] = input
            .chars()
            .map(Card::from_char)
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .unwrap();

        let hand_type = HandType::find(&cards);

        Ok(Self { hand_type, cards })
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input).unwrap();
    println!("{}", output);
}

fn process(input: &str) -> anyhow::Result<u32> {
    let mut hands = input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut parts| {
            let hand = parts
                .next()
                .with_context(|| "Cards not found")
                .map(Hand::parse)??;
            let bid = parts
                .next()
                .with_context(|| "Bid not found")
                .map(|bid| bid.parse::<u32>())??;
            Ok((hand, bid))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    let result = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank as u32 + 1) * bid)
        .sum::<u32>();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering() {
        assert!(Card::A > Card::Two);
        assert!(HandType::FiveOfAKind > HandType::HighCard);

        let hand1 = Hand::parse("AAAAA").unwrap();
        let hand2 = Hand::parse("AA8AA").unwrap();
        assert!(hand1 > hand2);

        let hand1 = Hand::parse("33332").unwrap();
        let hand2 = Hand::parse("2AAAA").unwrap();
        assert!(hand1 > hand2);
    }

    #[test]
    fn hand_parsing() {
        assert_eq!(
            Hand::parse("AAAAA").unwrap().hand_type,
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::parse("AA8AA").unwrap().hand_type,
            HandType::FourOfAKind
        );
        assert_eq!(Hand::parse("23332").unwrap().hand_type, HandType::FullHouse);
        assert_eq!(
            Hand::parse("TTT98").unwrap().hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(Hand::parse("23432").unwrap().hand_type, HandType::TwoPair);
        assert_eq!(Hand::parse("A23A4").unwrap().hand_type, HandType::OnePair);
        assert_eq!(Hand::parse("23456").unwrap().hand_type, HandType::HighCard);

        assert_eq!(Hand::parse("T55J5").unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(Hand::parse("KTJJT").unwrap().hand_type, HandType::FourOfAKind);
        assert_eq!(Hand::parse("QQQJA").unwrap().hand_type, HandType::FourOfAKind);
    }
}
