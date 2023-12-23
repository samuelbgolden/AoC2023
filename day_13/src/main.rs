use std::{
    collections::HashMap,
    io::{stdin, BufReader, Read},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Item {
    Ash,
    Rock,
}

struct Pattern {
    pub map: HashMap<Point, Item>,
    pub size: Point,
}

impl Pattern {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            size: Point::new(0, 0),
        }
    }

    fn insert(&mut self, key: Point, val: Item) {
        if (key.x + 1) > self.size.x {
            self.size.x = key.x + 1;
        }
        if (key.y + 1) > self.size.y {
            self.size.y = key.y + 1;
        }

        let err_str = format!("tried to overwrite pattern val: {:?}, {:?}", key, val);
        match self.map.insert(key, val) {
            Some(_) => panic!("{}", err_str),
            None => (),
        }
    }

    fn get(&self, p: &Point) -> &Item {
        match self.map.get(p) {
            Some(i) => i,
            None => panic!("asked for nonexistent map val at point {:?}", p),
        }
    }

    fn check_cols_equal(&self, x0: usize, x1: usize) -> bool {
        (0..self.size.y).all(|y| *self.get(&Point::new(x0, y)) == *self.get(&Point::new(x1, y)))
    }

    fn check_rows_equal(&self, y0: usize, y1: usize) -> bool {
        (0..self.size.x).all(|x| *self.get(&Point::new(x, y0)) == *self.get(&Point::new(x, y1)))
    }

    fn find_sym_line(&self) -> (usize, usize) {
        for (p0, p1) in Vec::from_iter(0..self.size.x)
            .as_slice()
            .windows(2)
            .map(|w| (Point::new(w[0], 0), Point::new(w[1], 0)))
        {
            let mut found = false;
            for (c0, c1) in (0..(p0.x + 1)).rev().zip(p1.x..self.size.x) {
                if !self.check_cols_equal(c0, c1) {
                    break;
                }
                if c0 == 0 || c1 == (self.size.x - 1) {
                    found = true;
                    break;
                }
            }
            if found {
                return (0, p0.x);
            }
        }
        for (p0, p1) in Vec::from_iter(0..self.size.y)
            .as_slice()
            .windows(2)
            .map(|w| (Point::new(0, w[0]), Point::new(0, w[1])))
        {
            let mut found = false;
            for (r0, r1) in (0..(p0.y + 1)).rev().zip(p1.y..self.size.y) {
                if !self.check_rows_equal(r0, r1) {
                    break;
                }
                if r0 == 0 || r1 == (self.size.y - 1) {
                    found = true;
                    break;
                }
            }
            if found {
                return (1, p0.y);
            }
        }
        panic!("didn't find symmetry!");
    }
}

fn main() {
    let mut inp: String = String::new();
    if let Ok(read_size) = BufReader::new(stdin().lock()).read_to_string(&mut inp) {
        assert!(read_size > 0);
    }

    inp = inp.replace("\r", "");
    let patterns = inp.split("\n\n");

    println!(
        "answer: {}",
        patterns
            .map(|p| {
                let mut pattern: Pattern = Pattern::new();
                for (y, row) in p.split("\n").enumerate() {
                    for (x, chr) in row.char_indices() {
                        pattern.insert(
                            Point::new(x, y),
                            match chr {
                                '.' => Item::Ash,
                                '#' => Item::Rock,
                                _ => panic!("unexpected char in pattern string"),
                            },
                        );
                    }
                }

                let (axis, idx) = pattern.find_sym_line();
                //println!("{:?}", sym_line);
                match axis {
                    1 => (idx + 1) * 100,
                    0 => idx + 1,
                    other => panic!("unexpected axis: {:?}", other),
                }
            })
            .sum::<usize>()
    );
}
