use itertools::Itertools;
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();

    let turns: String = lines.next().expect("first line").expect("read line");
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    lines.skip(1).for_each(|l| match l {
        Ok(line) => {
            let mut iter = line.split(" = ");
            let key: &str = iter.next().expect("first elem");
            let tuple_str: &str = &iter
                .next()
                .expect("tuple string")
                .replace("(", "")
                .replace(")", "");

            let conn_tup: (String, String) = tuple_str
                .split(", ")
                .map(str::to_string)
                .collect_tuple()
                .expect("build tuple");

            map.insert(key.to_string(), conn_tup);
        }
        Err(_) => panic!(),
    });

    let mut count = 0u32;
    let mut curr: &String = &String::from("AAA");
    for turn in turns.chars().cycle() {
        if curr == "ZZZ" {
            println!("turn count: {}", count);
            break;
        }
        curr = match turn {
            'L' => &map.get(curr).expect("current location in map").0,
            'R' => &map.get(curr).expect("current location in map").1,
            _ => panic!(),
        };
        count += 1;
    }
}
