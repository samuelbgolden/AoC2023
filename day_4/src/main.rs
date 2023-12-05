use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead, BufReader},
};

struct CardMap {
    card_counts: HashMap<u32, u32>,
}
impl CardMap {
    fn add_or_insert(&mut self, key: u32, val: u32) {
        match self.card_counts.get_mut(&key) {
            Some(stored) => *stored += val,
            None => {
                let _ = self.card_counts.insert(key, val);
            }
        }
    }

    fn get(&mut self, key: u32) -> u32 {
        match self.card_counts.get(&key) {
            Some(val) => *val,
            None => {
                let _ = self.card_counts.insert(key, 0);
                0
            }
        }
    }
}

fn main() {
    // read input
    let lines = BufReader::new(stdin().lock()).lines();

    let mut card_map = CardMap {
        card_counts: HashMap::new(),
    };

    let card_count: u32 = lines
        .map(|line| match line {
            Ok(line) => {
                let mut l = line.split(':');

                let card_num: u32 = l
                    .next() // gets "Card #"
                    .unwrap()
                    .split(' ') // then ["Card", "#"]
                    .last() // then "#"
                    .unwrap()
                    .parse() // and finally #
                    .unwrap();

                // add the original card
                card_map.add_or_insert(card_num, 1);

                if let [winners, candidates] =
                    l.next().unwrap().split('|').collect::<Vec<&str>>()[..]
                {
                    // read winning nums
                    let winning_nums: HashSet<u32> = winners
                        .split(' ')
                        .filter_map(|num| num.parse::<u32>().ok())
                        .collect::<HashSet<u32>>();

                    // check scratch off nums for matches
                    let matches: u32 = candidates
                        .split(' ')
                        .filter_map(|num| num.parse::<u32>().ok())
                        .filter(|num| winning_nums.contains(num))
                        .count() as u32;

                    assert!(winning_nums.len() == 10);

                    let curr_card_count = card_map.get(card_num);

                    // add the count of the current card to the next <matches> cards counts
                    ((card_num + 1)..=(card_num + matches))
                        .into_iter()
                        .for_each(|future_card_num| {
                            card_map.add_or_insert(future_card_num, curr_card_count)
                        });
                    return curr_card_count;
                } else {
                    return 0;
                }
            }
            Err(_) => 0,
        })
        .sum();

    println!("card count: {}", card_count);
}
