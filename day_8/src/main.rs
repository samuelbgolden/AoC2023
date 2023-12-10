use itertools::Itertools;
use num_integer::Integer;
use std::{
    collections::HashMap,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();

    let turns: String = lines.next().expect("first line").expect("read line");
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut tracked_nodes: Vec<String> = vec![];

    // build map and identify starting nodes
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
            if key.ends_with('A') {
                tracked_nodes.push(key.to_string());
            }
        }
        Err(_) => panic!(),
    });

    println!("starting nodes: {:?}", tracked_nodes);

    // count the turns to the first z node for each node that ends with a
    let mut first_hits: Vec<u64> = vec![];
    for node in tracked_nodes.iter_mut() {
        let mut count = 0;
        for turn in turns.chars().cycle() {
            *node = match turn {
                'L' => map
                    .get(node)
                    .expect("current location in map")
                    .0
                    .to_string(),
                'R' => map
                    .get(node)
                    .expect("current location in map")
                    .1
                    .to_string(),
                _ => panic!(),
            };
            count += 1;
            if node.chars().last().expect("get last char") == 'Z' {
                println!("{}: {}", node, count);
                first_hits.push(count);
                break;
            }
        }
    }

    // find the least common multiple of all the counts
    println!(
        "lcm of all: {}",
        first_hits.iter().fold(1i64, |acc, i| acc.lcm(&(*i as i64)))
    );
}
