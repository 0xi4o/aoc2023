use crate::Card;

pub fn process(input: &str) -> u32 {
    let matches = input
        .lines()
        .map(|line| {
            let (_, card) = Card::parse(line).unwrap();
            let matches = card.calculate_matches();

            matches
        })
        .collect::<Vec<_>>();

    let mut counts: Vec<u32> = vec![1; input.lines().count()];
    for (i, num) in matches.iter().enumerate() {
        for _ in 0..(counts[i] as usize) {
            for j in 0..(*num as usize) {
                counts[i+j+1] += 1;
            }
        }
    }

    counts.iter().sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(input);
        assert_eq!(result, 30);
    }
}
