use crate::{parse_races};

pub fn process(input: &str) -> u32 {
    let (_, races) = parse_races(input).expect("should be a tuple");
    let speed: u32 = 1;
    let output: u32 = races
        .iter()
        .map(|(time, distance)| {
            let options = (0..=(*time as usize))
                .filter_map(|t| {
                    // distance = (time - t) * t * speed
                    let distance_travelled = (time - t as u32) * (t as u32) * speed;
                    if distance_travelled > *distance {
                        Some(t as u32)
                    } else {
                        None
                    }
                })
                .count();
            options as u32
        })
        .product();

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(input);
        assert_eq!(result, 288);
    }
}