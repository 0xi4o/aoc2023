use crate::{parse_race};

pub fn process(input: &str) -> u64 {
    let (_, (time, distance)) = parse_race(input).expect("should be a tuple");
    let output = (0..=(time as usize))
        .filter_map(|t| {
            // distance = (time - t) * t * speed
            let distance_travelled = (time - t as u64) * (t as u64) * 1;
            if distance_travelled > distance {
                Some(t as u32)
            } else {
                None
            }
        })
        .count();

    output as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(input);
        assert_eq!(result, 71503);
    }
}
