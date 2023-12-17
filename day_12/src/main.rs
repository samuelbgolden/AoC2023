use itertools::Itertools;
use std::{
    io::{stdin, BufRead, BufReader},
    iter,
};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Condition {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            _ => Self::Unknown,
        }
    }
}

fn print_conds(conds: &Vec<Condition>) {
    println!(
        "{}",
        conds
            .iter()
            .map(|c| match c {
                Condition::Operational => '.',
                Condition::Damaged => '#',
                Condition::Unknown => '?',
            })
            .join("")
    );
}

fn is_possible(schematic: &Vec<Condition>, guess: &Vec<Condition>) -> bool {
    for (s, g) in schematic.iter().zip_eq(guess.iter()) {
        match (s, g) {
            (Condition::Operational, Condition::Damaged) => return false,
            (Condition::Damaged, Condition::Operational) => return false,
            (_, Condition::Unknown) => panic!("Unknown condition in guess!"),
            _ => (),
        }
    }
    return true;
}

fn gen_guesses(schematic: &Vec<Condition>, group_sizes: &Vec<usize>) -> Vec<Vec<Condition>> {
    let mut cumul_sum: usize = 0;
    let mut idxs: Vec<usize> = vec![];
    for g in group_sizes.iter() {
        idxs.push(cumul_sum);
        cumul_sum += g + 1;
    }

    let groups_shift_length: usize =
        schematic.len() - ((group_sizes.len() - 1) + group_sizes.iter().sum::<usize>());

    // get the cartesian product of all possible valid starting indices for each group
    (0..group_sizes.len())
        .map(|i| idxs[i]..=(idxs[i] + groups_shift_length))
        .multi_cartesian_product()
        // filter out the items that would cause groups to overlap or be too close
        .filter(|group_start_idxs| {
            for j in 1..group_start_idxs.len() {
                if group_start_idxs[j] <= (group_start_idxs[j - 1] + group_sizes[j - 1]) {
                    return false;
                }
            }
            true
        })
        // generate the spring condition list for each valid set of indices
        .map(|group_start_idxs| {
            let mut guess: Vec<Condition> = vec![];
            let mut j: usize = 0;
            for (group_idx, group_start_idx) in group_start_idxs.iter().enumerate() {
                guess.extend(iter::repeat(Condition::Operational).take(group_start_idx - j));
                guess.extend(iter::repeat(Condition::Damaged).take(group_sizes[group_idx]));
                j = group_start_idx + group_sizes[group_idx];
            }
            guess
                .iter()
                .map(|c| *c)
                .pad_using(schematic.len(), |_| Condition::Operational)
                .collect_vec()
        })
        .collect_vec()
}

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let sum: usize = lines
        .filter(|l| l.is_ok())
        .map(|l| {
            let line = l.unwrap();
            let (spring_string, group_string) = line.split(' ').collect_tuple::<(_, _)>().unwrap();
            let schematic: Vec<Condition> =
                spring_string.chars().map(Condition::from).collect_vec();
            let groups: Vec<usize> = group_string
                .split(',')
                .filter_map(|c| c.parse::<usize>().ok())
                .collect_vec();

            let guesses: Vec<Vec<Condition>> = gen_guesses(&schematic, &groups);

            guesses
                .iter()
                .filter(|guess| is_possible(&schematic, guess))
                .count()
        })
        .sum();
    println!("answer: {}", sum);
}
