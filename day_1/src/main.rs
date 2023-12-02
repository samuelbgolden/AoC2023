use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const PATTERNS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const REV_PATTERNS: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

const RADIX: u32 = 10;

fn main() {
    if let Ok(recs) = read_lines(".\\resources\\input.txt") {
        let sum: usize = recs
            .map(|rec| {
                (find_first_digit(rec.as_ref().unwrap().clone(), false).to_string()
                    + &find_first_digit(rec.unwrap().to_string(), true).to_string())
                    .parse::<usize>()
                    .unwrap()
            })
            .sum::<usize>();

        println!("{}", sum);
    }
}

fn find_first_digit(mut string: String, rev: bool) -> usize {
    let patterns;
    if rev {
        string = string.chars().rev().collect::<String>();
        patterns = REV_PATTERNS;
    } else {
        patterns = PATTERNS;
    }

    let mut i = 0;

    loop {
        let c = string.chars().nth(i).unwrap();

        if c.is_digit(RADIX) {
            return c.to_digit(RADIX).unwrap() as usize;
        }

        for pattern in patterns.iter() {
            if string.get(i..).unwrap().starts_with(pattern) {
                return patterns.iter().position(|p| p == pattern).unwrap();
            }
        }

        i = i + 1;
    }
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
