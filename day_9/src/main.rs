use std::io::{stdin, BufRead, BufReader};

use sequence::{Int, Sequence};
mod sequence;

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let mut seqs: Vec<Sequence> = vec![];
    lines.for_each(|line| {
        seqs.push(Sequence::new(
            line.expect("read line")
                .split(' ')
                .filter_map(|n| n.parse::<Int>().ok())
                .collect(),
        ))
    });

    let next_vals_sum: Int = seqs
        .iter_mut()
        .map(|s| {
            let next = s.extrapolate_prev();
            //println!("{}\n", s);
            next
        })
        .sum();

    println!("answer: {}", next_vals_sum);
}
