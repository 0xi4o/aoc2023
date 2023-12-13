pub mod part1;
pub mod part2;

pub enum Hands {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub struct Games {
    hand: Hands,
    bid: u32
}