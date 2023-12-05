use std::{
    collections::HashSet,
    io::{stdin, BufRead, BufReader},
};

fn main() {
    // read input
    let lines = BufReader::new(stdin().lock()).lines();
    let points: usize = lines
        .filter_map(|line| match line {
            Ok(l) => {
                if let [winners, candidates] = l
                    .split(':')
                    .last()
                    .unwrap()
                    .split('|')
                    .collect::<Vec<&str>>()[..]
                {
                    let winning_nums: HashSet<u32> = winners
                        .split(' ')
                        .filter_map(|num| num.parse::<u32>().ok())
                        .collect::<HashSet<u32>>();
                    return Some(
                        candidates
                            .split(' ')
                            .filter_map(|num| num.parse::<u32>().ok())
                            .filter(|num| winning_nums.contains(num))
                            .count(),
                    );
                } else {
                    println!("couldn't split on '|' for line '{}'", l);
                    return None;
                };
            }
            Err(_) => None,
        })
        .map(get_score)
        .sum();
    println!("points: {}", points);
}

fn get_score(match_count: usize) -> usize {
    match match_count {
        0 => 0,
        _ => 2usize.pow((match_count - 1) as u32),
    }
}
