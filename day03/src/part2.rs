pub fn process(input: &str) -> u32 {
    let _lines = input.lines().collect::<Vec<&str>>();
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        let result = process(input);
        assert_eq!(result, 0);
    }
}
