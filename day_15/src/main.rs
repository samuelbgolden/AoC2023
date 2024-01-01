use std::io::{stdin, BufReader, Read};

fn main() {
    let mut inp: String = String::new();
    if let Ok(read_size) = BufReader::new(stdin().lock()).read_to_string(&mut inp) {
        assert!(read_size > 0);
    }

    inp = inp.replace("\r", "");
    inp = inp.replace("\n", "");

    let answer: usize = inp.split(",").map(|v| hash_alg(v)).sum();
    println!("answer: {}", answer);
}

fn hash_alg(s: &str) -> usize {
    let mut x = 0usize;
    for val in s.chars().map(|c| c as usize) {
        x = ((x + val) * 17) % 256;
    }
    x
}
