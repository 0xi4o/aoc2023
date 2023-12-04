use std::collections::BTreeMap;

#[derive(Clone, Debug)]
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

    // list of numbers with start and end coordinates
    let mut numbers: Vec<(Coords, Coords, u32)> = vec![];
    // buffer for collecting each part number in the engine schematic
    let mut num: Vec<u32> = vec![];
    for (coords, value) in map.iter() {
        if let Value::Number(v) = value {
            num.push(*v);
        } else {
            let v = num.iter().fold(0, |acc, e| acc * 10 + e);
            // prevents a bunch of zeroes being inserted into our part numbers list
            if v != 0 {
                // calculate start and end coordinates for each number
                numbers.push((
                    Coords { x: coords.x, y: if coords.y >= num.len() { coords.y - num.len() } else { coords.y } },
                    Coords { x: coords.x, y: if coords.y > 0 { coords.y - 1 } else { coords.y } },
                    v
                ));
            }
            num = vec![];
            continue;
        }
    }
    let output: u32 = numbers
        .iter()
        .map(|(start, end, v)| {
            let mut coords_to_check: Vec<Coords> = vec![];
            // (2, 2) to (2, 3): 35
            // coords to check for symbols: (1, 1), (1, 2), (1, 3), (1, 4), (2, 1), (2, 4), (3, 1), (3, 2), (3, 3), (3, 4)
            if start.x > 0 && start.y > 0 {
                for i in start.x - 1..=end.x + 1 {
                    for j in start.y - 1..=end.y + 1 {
                        coords_to_check.push(Coords { x: i, y: j });
                    }
                }
            } else if start.x > 0 {
                for i in start.x - 1..=end.x + 1 {
                    for j in 0..=end.y + 1 {
                        coords_to_check.push(Coords { x: i, y: j });
                    }
                }
            } else if start.y > 0 {
                for i in 0..=end.x + 1 {
                    for j in start.y - 1..=end.y + 1 {
                        coords_to_check.push(Coords { x: i, y: j });
                    }
                }
            } else {
                for i in 0..=end.x + 1 {
                    for j in 0..=end.y + 1 {
                        coords_to_check.push(Coords { x: i, y: j });
                    }
                }
            }
            // remove start and end from coords_to_check because we already know they're numbers
            // one caveat here is if the number is 3 or more digits, we're checking the middle digits
            // but it's fine for now
            let filtered_coords = coords_to_check
                .iter()
                .filter(|coords| {
                    **coords != *start && **coords != *end
                })
                .filter(|coords| {
                    let rows_len = input.lines().collect::<Vec<&str>>().len();
                    coords.x < rows_len
                })
                .map(|coords| *coords)
                .collect::<Vec<Coords>>();
            (filtered_coords, *v)
        })
        .filter_map(|(coords_to_check, v)| {
            let part_numbers: u32 = coords_to_check.iter().filter_map(|coords| {
                let map_val = map.get(coords).expect("should be a Value");
                let res = match map_val {
                    Value::Symbol(_) => v,
                    _ => 0
                };
                Some(res)
            }).collect::<Vec<u32>>().iter().sum();
            Some(part_numbers)
        })
        // .inspect(|v| {
        //     println!("{:?}", v);
        // })
        .sum();

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = process(input);
        assert_eq!(result, 4361);
    }
}