use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut lines = BufReader::new(stdin().lock()).lines();
    let times: Vec<i64> = lines
        .next()
        .expect("first line present")
        .expect("retrieve line of times")
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    let distances: Vec<i64> = lines
        .next()
        .expect("first line present")
        .expect("retrieve line of times")
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    let mut product: i64 = 1;

    for (time, record_dist) in times.iter().zip(distances.iter()) {
        // you can set up the values as "dist = held_time * (race_time - held_time)", or:
        // "held_time**2 - race_time*held_time + dist = 0", and then calc roots of that parabola
        let (l, h) = get_roots(1f64, -(*time as f64), *record_dist as f64);
        let (low, high) = (l.ceil() as i64, h.floor() as i64);

        let mut p = high - low + 1;
        if ((h as i64 * *time) - (h.powi(2) as i64)) == *record_dist {
            p -= 2;
        }

        product *= p;
    }

    println!("product: {}", product);
}

fn get_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let term1 = ((b.powi(2) - (4f64 * a * c)) as f64).sqrt();
    let term2 = 2f64 * a;
    return (((-b) - term1) / term2, ((-b) + term1) / term2);
}
