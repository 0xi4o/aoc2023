use crate::helpers::parse_digits;

pub fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;
    for line in lines {
        let digits = parse_digits(line);
        let first_digit = &digits[..].first().unwrap();
        let last_digit = &digits[..].last().unwrap();
        let value = format!("{first_digit}{last_digit}");
        let num: u32 = value.parse().unwrap();
        sum += num;
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = process(input);
        assert_eq!(result, 142);
    }
}