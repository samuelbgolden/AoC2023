mod hand;
mod rank;
use hand::Hand;
use itertools::Itertools;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let hands: Vec<(Hand, u64)> = lines
        .map(|record| match record {
            Ok(line) => {
                let (hand_str, bid_str): (&str, &str) =
                    line.split(" ").collect_tuple().expect("build tuple");
                (
                    Hand::from(hand_str.to_string()),
                    bid_str.parse::<u64>().expect("parsed u64"),
                )
            }
            _ => panic!(),
        })
        .collect_vec();

    println!(
        "total winnings: {}",
        hands
            .iter()
            .sorted()
            .zip(1..)
            .map(|(val, i)| {
                println!("hand with rank {}: {}", i, val.0);
                val.1 * (i as u64)
            })
            .sum::<u64>()
    );
}
