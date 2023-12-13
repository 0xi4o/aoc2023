pub fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let result = process(input);
        assert_eq!(result, 6440);
    }
}