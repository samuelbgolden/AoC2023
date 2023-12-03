use std::collections::{HashMap, HashSet};
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

    let mut gear_counter: u32 = 0;
    let mut gear_map: HashMap<Point, HashSet<u32>> = HashMap::new();

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
                } else if char == '*' {
                    gear_counter += 1;
                    // for each point surrounding the gear, add the gear's id to the map
                    AREA_DELTAS.iter().for_each(|(dx, dy)| {
                        let p = Point {
                            x: x + dx,
                            y: y + dy,
                        };
                        match gear_map.get_mut(&p) {
                            Some(gear_set) => drop(gear_set.insert(gear_counter)),
                            None => drop(gear_map.insert(p, HashSet::from([gear_counter]))),
                        };
                    });
                }
            }
        }
    }

    let mut gear_hits: HashMap<u32, HashSet<u32>> = HashMap::new();

    // check every point between the start and end of each number for a corresponding
    // flag in the symbol map; sum those with a hit
    numbers.iter().for_each(|(start, end, val)| {
        (start.x..=end.x)
            .zip(iter::repeat(start.y))
            .map(|(x, y)| Point { x, y })
            // for each point in the boundaries of the number, check which gears are adjacent in the gear map
            .for_each(|p| match gear_map.get(&p) {
                // if there are adjacent gears, add the value of the number to the gear_hits map for each adjacent gear
                Some(adj_gears) => {
                    adj_gears
                        .iter()
                        .for_each(|adj_gear| match gear_hits.get_mut(adj_gear) {
                            Some(ratio_list) => {
                                let _ = ratio_list.insert(val.parse::<u32>().unwrap());
                            }
                            None => {
                                drop(gear_hits.insert(
                                    *adj_gear,
                                    HashSet::from([val.parse::<u32>().unwrap()]),
                                ))
                            }
                        })
                }
                None => (),
            })
    });

    let sum: u32 = gear_hits
        .values()
        .filter_map(|ratios| match ratios.len() {
            2 => {
                let rs = ratios.iter().collect::<Vec<&u32>>();
                Some(rs[0] * rs[1])
            }
            _ => None,
        })
        .sum();

    println!("{:?}", sum);
}
