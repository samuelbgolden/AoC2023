use std::{
    collections::HashMap,
    io::{stdin, BufReader, Read},
};

fn main() {
    let mut inp: String = String::new();
    if let Ok(read_size) = BufReader::new(stdin().lock()).read_to_string(&mut inp) {
        assert!(read_size > 0);
    }

    inp = inp.replace("\r", "");
    inp = inp.replace("\n", "");

    let mut boxes: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

    inp.split(",").for_each(|v| {
        let focal_len: usize;
        let box_num: usize;
        let label: &str;
        if v.contains("=") {
            let t = v.split("=").collect::<Vec<&str>>();
            label = t[0];
            box_num = hash_alg(label);
            focal_len = t[1].parse().expect("parse focal len");
            boxes
                .entry(box_num)
                .and_modify(|lenses| {
                    if let Some(idx) = lenses.iter().position(|lens| lens.0.eq(&label.to_string()))
                    {
                        lenses.remove(idx);
                        lenses.insert(idx, (label.to_string(), focal_len));
                    } else {
                        lenses.push((label.to_string(), focal_len));
                    }
                })
                .or_insert(vec![(label.to_string(), focal_len)]);
        } else {
            let t = v.trim_end_matches('-');
            label = t;
            box_num = hash_alg(label);
            boxes.entry(box_num).and_modify(|lenses| {
                if let Some(idx) = lenses.iter().position(|lens| lens.0.eq(&label.to_string())) {
                    lenses.remove(idx);
                }
            });
        }
    });

    let mut answer: usize = 0;
    for (box_num, lenses) in boxes.iter() {
        for (i, (_, focal_len)) in lenses.iter().enumerate() {
            answer += (box_num + 1) * (i + 1) * (*focal_len);
        }
    }

    println!("answer: {}", answer);
}

fn hash_alg(s: &str) -> usize {
    let mut x = 0usize;
    for val in s.chars().map(|c| c as usize) {
        x = ((x + val) * 17) % 256;
    }
    x
}
