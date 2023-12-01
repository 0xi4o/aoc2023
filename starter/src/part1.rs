pub fn process(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    todo!();
    "".to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "";
        let result = process(input);
        assert_eq!(result, "".to_string());
    }
}