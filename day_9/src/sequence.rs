use itertools::Itertools;
use std::fmt::Display;
pub type Int = i64;

pub struct Sequence {
    sequences: Vec<Vec<Int>>,
    seq_starting_vals: Vec<Int>,
    seq_final_vals: Vec<Int>,
    zero_layer_built: bool,
}

impl Sequence {
    pub fn new(values: Vec<Int>) -> Self {
        Self {
            zero_layer_built: !values.iter().any(|x| *x != 0),
            seq_starting_vals: vec![values[0]],
            seq_final_vals: vec![*values.last().expect("has value")],
            sequences: vec![values],
        }
    }

    fn next_layer(&mut self) {
        if self.zero_layer_built {
            println!("zero layer already built!");
            return;
        }
        let mut new_layer: Vec<Int> = vec![];
        let mut all_zero = true;
        self.sequences
            .last()
            .expect("has 1 seq")
            .as_slice()
            .windows(2)
            .for_each(|win| {
                let diff = win[1] - win[0];
                if diff != 0 {
                    all_zero = false;
                }
                new_layer.push(diff);
            });
        self.zero_layer_built = all_zero;
        self.seq_starting_vals.push(new_layer[0]);
        self.seq_final_vals
            .push(*new_layer.last().expect("has a value"));
        self.sequences.push(new_layer);
    }

    pub fn extrapolate_next(&mut self) -> Int {
        while !self.zero_layer_built {
            self.next_layer();
        }
        return self.seq_final_vals.iter().rev().fold(0, |acc, n| acc + n);
    }

    pub fn extrapolate_prev(&mut self) -> Int {
        while !self.zero_layer_built {
            self.next_layer();
        }
        return self
            .seq_starting_vals
            .iter()
            .rev()
            .fold(0, |acc, n| n - acc);
    }
}

impl Display for Sequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.sequences
                .iter()
                .map(|seq| seq.iter().join("\t"))
                .join("\n")
        )
    }
}
