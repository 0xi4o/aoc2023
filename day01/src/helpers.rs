#[derive(Clone, Copy)]
pub enum Digits {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl Digits {
    pub fn from(input: &str) -> u32 {
        let result = match input {
            "one" => Self::One,
            "two" => Self::Two,
            "three" => Self::Three,
            "four" => Self::Four,
            "five" => Self::Five,
            "six" => Self::Six,
            "seven" => Self::Seven,
            "eight" => Self::Eight,
            "nine" => Self::Nine,
            _ => panic!("unsupported digit")
        };

        result as u32
    }
}

pub fn parse_digits(input: &str) -> Vec<u32> {
    let lookup_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut digits: Vec<(usize, u32)> = vec![];
    for digit in lookup_digits {
        let mut result: Vec<_> = input.match_indices(digit).collect();
        if result.len() > 0 {
            let mut parsed_result = result.iter().map(|t| (t.0, Digits::from(t.1))).collect::<Vec<(usize, u32)>>();
            digits.append(&mut parsed_result);
        }
    }
    let mut res2: Vec<_> = input.match_indices(char::is_numeric).collect();
    if res2.len() > 0 {
        let mut parsed_result = res2.iter().map(|t| (t.0, t.1.parse::<u32>().unwrap())).collect::<Vec<(usize, u32)>>();
        digits.append(&mut parsed_result);
    }
    digits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    digits.iter().map(|t| t.1).collect::<Vec<u32>>()
}