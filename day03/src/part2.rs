use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Value {
    Empty,
    Number(u32),
    Symbol(char),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Coords {
    x: usize,
    y: usize,
}

pub fn process(input: &str) -> u32 {
    let line_len: usize = input.lines().nth(0).expect("should be a &str").len();
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().map(move |(y, character)| {
                let val = match character {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(c.to_digit(10).expect("should be a u32"))
                    }
                    c => Value::Symbol(c)
                };
                (Coords { x, y }, val)
            })
        })
        .collect::<BTreeMap<Coords, Value>>();

    let mut start = Coords { x: 0, y: 0 };
    let mut end = Coords { x: 0, y: 0 };
    let mut is_end_of_line = false;
    // buffer for collecting each part number in the engine schematic
    let mut buf: Vec<u32> = vec![];
    // list of numbers with start and end coordinates
    let numbers = map
        .iter()
        .filter_map(|(key, value)| {
            match value {
                Value::Number(v) => {
                    buf.push(*v);
                    if key.y == line_len - 1 {
                        // check if the current position is the end of a line
                        is_end_of_line = true;
                    }
                    None
                }
                _ => {
                    let num = buf.iter().fold(0, |acc, e| acc * 10 + e);
                    // // prevents a bunch of zeroes being inserted into our part numbers list
                    if num != 0 {
                        // calculate start and end coordinates for each number
                        if is_end_of_line {
                            start = Coords {
                                x: key.x - 1,
                                y: line_len - buf.len(),
                            };
                            end = Coords {
                                x: key.x - 1,
                                y: line_len - 1,
                            };
                        } else {
                            start = Coords {
                                x: key.x,
                                y: key.y - buf.len(),
                            };
                            end = Coords {
                                x: key.x,
                                y: key.y - 1,
                            };
                        }
                        buf = vec![];
                        is_end_of_line = false;
                        Some((start, end, num))
                    } else {
                        None
                    }
                }
            }
        })
        .inspect(|x| {
            println!("{:?}", x);
        })
        .collect::<Vec<(Coords, Coords, u32)>>();

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 467835);
    }
}
