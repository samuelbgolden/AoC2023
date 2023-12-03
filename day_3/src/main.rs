use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, BufRead, BufReader};
use std::iter;

const RADIX: u32 = 10;

const AREA_DELTAS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // read input
    let input = BufReader::new(stdin().lock()).lines();
    let lines: Vec<Result<String, std::io::Error>> = input.collect();

    let mut symbol_map: HashMap<Point, bool> = HashMap::new();

    // Vec<(start, end, value)>
    let mut numbers: Vec<(Point, Point, String)> = vec![];

    // iterate over each row
    for (line, y) in lines.iter().zip(0i32..) {
        if let Ok(row) = line {
            // iterate over each char in the row
            for (char, x) in row.chars().zip(0i32..) {
                // if digit, record number found
                if char.is_digit(RADIX) {
                    if let Some(prev_num) = numbers.last_mut() {
                        // if previous char was a digit, continue the previous number recorded
                        if prev_num.1.x == (x - 1) && prev_num.0.y == y {
                            prev_num.1.x = x;
                            let _ = prev_num.2.write_char(char);
                            continue;
                        }
                    }
                    numbers.push((Point { x, y }, Point { x, y }, char.to_string()));
                } else if char != '.' {
                    // for each point surrounding the symbol, add flag to map
                    AREA_DELTAS.iter().for_each(|(dx, dy)| {
                        symbol_map.insert(
                            Point {
                                x: x + dx,
                                y: y + dy,
                            },
                            true,
                        );
                    });
                }
            }
        }
    }

    // check every point between the start and end of each number for a corresponding
    // flag in the symbol map; sum those with a hit
    let sum: u32 = numbers
        .iter()
        .filter_map(|(start, end, val)| {
            if (start.x..=end.x)
                .zip(iter::repeat(start.y))
                .map(|(x, y)| Point { x, y })
                .any(|p| *symbol_map.get(&p).unwrap_or(&false))
            {
                return Some(val);
            }
            None
        })
        .map(|val| val.parse::<u32>().unwrap())
        .sum();

    println!("{:?}", sum);
}
