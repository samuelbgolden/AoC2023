use itertools::Itertools;
use std::{
    io::{stdin, BufRead, BufReader},
    iter,
};

const SCHEMATIC_COPIES: usize = 1;

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

#[allow(dead_code)]
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

    let max_idxs: Vec<usize> = idxs.iter().map(|i| i + groups_shift_length).collect_vec();

    // filter check which start indices are actually possible for each group before doing cart. prod.
    let possible_start_idxs: Vec<Vec<usize>> = (0..group_sizes.len())
        .map(|i| {
            if i == 0 {
                let pattern: Vec<Condition> = iter::repeat(Condition::Damaged)
                    .take(group_sizes[i])
                    .chain(iter::once(Condition::Operational))
                    .collect_vec();
                (idxs[i]..=max_idxs[i])
                    .map(|j| {
                        let sub_schem = schematic[if j == 0 { 0 } else { j - 1 }..(if j == 0 {
                            0
                        } else {
                            j - 1
                        }
                            + group_sizes[i]
                            + 1)]
                            .iter()
                            .map(|c| *c)
                            .collect_vec();
                        //println!("a: {}, {}, {}", i, j, group_sizes[i]);
                        //print_conds(&pattern);
                        //print_conds(&sub_schem);
                        //print!("\n");
                        (j, sub_schem)
                    })
                    .filter(|(_, sub_schem)| is_possible(&sub_schem, &pattern))
                    .map(|(j, _)| j)
                    .collect_vec()
            } else if i == (group_sizes.len() - 1) {
                let pattern: Vec<Condition> = iter::once(Condition::Operational)
                    .chain(iter::repeat(Condition::Damaged).take(group_sizes[i]))
                    .collect_vec();
                (idxs[i]..=max_idxs[i])
                    .map(|j| {
                        let sub_schem = schematic
                            .iter()
                            .skip(j)
                            .take(group_sizes[i] + 1)
                            .map(|c| *c)
                            .collect_vec();
                        //println!("b: {}, {}, {}", i, j, group_sizes[i]);
                        //print_conds(&pattern);
                        //print_conds(&sub_schem);
                        //print!("\n");
                        (j, sub_schem)
                    })
                    .filter(|(_, sub_schem)| is_possible(&sub_schem, &pattern))
                    .map(|(j, _)| j)
                    .collect_vec()
            } else {
                let pattern: Vec<Condition> = iter::once(Condition::Operational)
                    .chain(iter::repeat(Condition::Damaged).take(group_sizes[i]))
                    .chain(iter::once(Condition::Operational))
                    .collect_vec();
                (idxs[i]..=max_idxs[i])
                    .map(|j| {
                        let sub_schem = schematic
                            .iter()
                            .skip(0.max(j - 1))
                            .take(group_sizes[i] + 2)
                            .map(|c| *c)
                            .collect_vec();
                        //println!("c: {}, {}, {}", i, j, group_sizes[i]);
                        //print_conds(&pattern);
                        //print_conds(&sub_schem);
                        //print!("\n");
                        (j, sub_schem)
                    })
                    .filter(|(_, sub_schem)| is_possible(&sub_schem, &pattern))
                    .map(|(j, _)| j)
                    .collect_vec()
            }
        })
        .collect_vec();

    // get the cartesian product of all possible valid starting indices for each group
    possible_start_idxs
        .iter()
        .multi_cartesian_product()
        // filter out the items that would cause groups to overlap or be too close
        .filter(|group_start_idxs| {
            for j in 1..group_start_idxs.len() {
                if group_start_idxs[j] <= &(group_start_idxs[j - 1] + group_sizes[j - 1]) {
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
                guess.extend(iter::repeat(Condition::Operational).take(*group_start_idx - j));
                guess.extend(iter::repeat(Condition::Damaged).take(group_sizes[group_idx]));
                j = *group_start_idx + group_sizes[group_idx];
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
                iter::repeat(spring_string.chars().map(Condition::from))
                    .take(SCHEMATIC_COPIES)
                    .fold(vec![], |mut acc, conds| {
                        acc.extend(conds);
                        acc.push(Condition::Unknown);
                        acc
                    });
            let groups: Vec<usize> = iter::repeat(
                group_string
                    .split(',')
                    .filter_map(|c| c.parse::<usize>().ok()),
            )
            .take(SCHEMATIC_COPIES)
            .flatten()
            .collect_vec();
            print_conds(&schematic);
            println!("{:?}", groups);

            let guesses: Vec<Vec<Condition>> = gen_guesses(&schematic, &groups);

            guesses
                .iter()
                .filter(|guess| is_possible(&schematic, guess))
                .count()
        })
        .sum();
    println!("answer: {}", sum);
}
