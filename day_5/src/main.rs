use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
struct Transformation {
    // (lower_bound, upper_bound exclusive, transformer)
    mappings: Vec<(i64, i64, i64)>,
}
impl Transformation {
    fn add_mapping(&mut self, dest_start: i64, source_start: i64, range_len: i64) {
        self.mappings.push((
            source_start,
            source_start + range_len,
            dest_start - source_start,
        ));
    }

    fn transform(&self, source_num: i64) -> i64 {
        for (lower_bound, upper_bound, transformer) in self.mappings.iter() {
            if source_num >= *lower_bound && source_num < *upper_bound {
                // source_num IS is the range of this mapping
                return source_num + transformer;
            }
        }
        source_num
    }
}

fn process() -> Option<i64> {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let seeds: Vec<i64> = lines
        .next()?
        .expect("to read line")
        .strip_prefix("seeds: ")
        .expect("'seeds: ' found in string")
        .split(' ')
        .map(|n| n.parse::<i64>().expect(&format!("parse i32 from {}", n)))
        .collect();

    let mut transformations: Vec<Transformation> = vec![];

    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            continue;
        } else if line.contains("map") {
            transformations.push(Transformation { mappings: vec![] });
        } else {
            let values: Vec<i64> = line
                .split(' ')
                .map(|n| n.parse::<i64>().expect(&format!("parse i32 from {}", n)))
                .collect();
            assert!(values.len() == 3);
            transformations
                .last_mut()
                .expect("items in transformations vec")
                .add_mapping(values[0], values[1], values[2]);
        }
    }

    seeds
        .iter()
        .map(|seed| transformations.iter().fold(*seed, |s, tr| tr.transform(s)))
        .min()
}

fn main() {
    println!("lowest location: {}", process().unwrap());
}
