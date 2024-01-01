use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::Display,
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
    Boulder,
    Wall,
    Empty,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Dish {
    pub map: HashMap<Point, Item>,
    pub size: Point,
}

impl Dish {
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

        let err_str = format!("tried to overwrite dish val: {:?}, {:?}", key, val);
        match self.map.insert(key, val) {
            Some(_) => panic!("{}", err_str),
            None => (),
        }
    }

    fn get(&self, p: &Point) -> &Item {
        match self.map.get(p) {
            Some(i) => i,
            None => panic!("asked for nonexistent val at point {:?}", p),
        }
    }

    fn roll_in_direction(&mut self, dir: Direction) {
        let mut section_marker;
        let mut section_boulders: Vec<Point> = vec![];
        let mut boulder_count;
        let mut pnt;

        let outer_range: Vec<usize>;
        let inner_range: Vec<usize>;

        match dir {
            Direction::North | Direction::South => {
                // works
                outer_range = (0..=(self.size.x - 1)).collect();
                inner_range = (0..=(self.size.y - 1)).collect();
            }
            Direction::East | Direction::West => {
                // does nothing
                outer_range = (0..=(self.size.y - 1)).collect();
                inner_range = (0..=(self.size.x - 1)).collect();
            }
        }

        for outer in outer_range {
            section_marker = 0;
            for inner in inner_range.clone() {
                if [Direction::North, Direction::South].contains(&dir) {
                    pnt = Point::new(outer, inner);
                } else {
                    pnt = Point::new(inner, outer);
                }
                match self.get(&pnt) {
                    Item::Boulder => section_boulders.push(pnt),
                    Item::Wall => {
                        boulder_count = section_boulders.len();

                        // clear previous boulder positions
                        for _ in 0..boulder_count {
                            self.map
                                .insert(section_boulders.pop().unwrap(), Item::Empty);
                        }

                        self.add_boulders_in_dir(&pnt, &dir, boulder_count, section_marker, outer);

                        // reset boulder/wall info
                        assert!(section_boulders.is_empty());
                        section_marker = inner + 1;
                    }
                    Item::Empty => (),
                }
            }

            if [Direction::North, Direction::South].contains(&dir) {
                pnt = Point::new(outer, self.size.y);
            } else {
                pnt = Point::new(self.size.x, outer);
            }

            boulder_count = section_boulders.len();
            for _ in 0..section_boulders.len() {
                self.map
                    .insert(section_boulders.pop().unwrap(), Item::Empty);
            }

            self.add_boulders_in_dir(&pnt, &dir, boulder_count, section_marker, outer);
        }
    }

    fn add_boulders_in_dir(
        &mut self,
        pnt: &Point,
        dir: &Direction,
        boulder_count: usize,
        section_marker: usize,
        outer: usize,
    ) {
        let mut p;
        match dir {
            Direction::North => {
                for i in (section_marker)..(section_marker + boulder_count) {
                    p = Point::new(outer, i);
                    self.map.insert(p, Item::Boulder);
                }
            }
            Direction::East => {
                for i in (pnt.x - boulder_count)..(pnt.x) {
                    p = Point::new(i, outer);
                    self.map.insert(p, Item::Boulder);
                }
            }
            Direction::South => {
                for i in (pnt.y - boulder_count)..(pnt.y) {
                    p = Point::new(outer, i);
                    self.map.insert(p, Item::Boulder);
                }
            }
            Direction::West => {
                for i in (section_marker)..(section_marker + boulder_count) {
                    p = Point::new(i, outer);
                    self.map.insert(p, Item::Boulder);
                }
            }
        }
    }

    fn count_boulders_in_row(&self, row: usize) -> usize {
        (0..self.size.x)
            .map(|x| self.get(&Point::new(x, row)))
            .filter(|i| **i == Item::Boulder)
            .count()
    }

    fn calc_north_load(&self) -> usize {
        (0..self.size.y)
            .map(|row| self.count_boulders_in_row(row) * (self.size.y - row))
            .sum::<usize>()
    }
}

impl Display for Dish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = self
            .map
            .iter()
            .sorted_by(|(p1, _), (p2, _)| Ord::cmp(&((p1.y * 100) + p1.x), &((p2.y * 100) + p2.x)))
            .map(|(_, i)| i)
            .chunks(self.size.x)
            .into_iter()
            .map(|chunk| {
                chunk
                    .map(|i| match i {
                        Item::Boulder => 'O',
                        Item::Wall => '#',
                        Item::Empty => '.',
                    })
                    .join(" ")
            })
            .join("\n");
        write!(f, "{}", t)
    }
}

fn main() {
    let mut inp: String = String::new();
    if let Ok(read_size) = BufReader::new(stdin().lock()).read_to_string(&mut inp) {
        assert!(read_size > 0);
    }
    inp = inp.replace("\r", "");

    let mut dish: Dish = Dish::new();
    for (y, row) in inp.split("\n").enumerate() {
        if row == "" {
            continue;
        }
        dish.size.y = dish.size.y.max(y + 1);
        for (x, c) in row.char_indices() {
            dish.size.x = dish.size.x.max(x + 1);
            dish.insert(
                Point::new(x, y),
                match c {
                    'O' => Item::Boulder,
                    '#' => Item::Wall,
                    '.' => Item::Empty,
                    other => panic!("what the heck is '{}'?", other),
                },
            )
        }
    }

    println!("og map:\n{}", dish);

    // get sample of iterations to find pattern in
    let mut north_loads: Vec<usize> = vec![];
    let trim_sample_start: usize = 100;
    let sample_iterations: usize = 1000;
    for _ in 0..sample_iterations {
        for dir in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            dish.roll_in_direction(dir);
        }
        north_loads.push(dish.calc_north_load());
    }

    let trimmed_loads = north_loads.iter().skip(trim_sample_start).collect_vec();

    let mut pattern_map: HashMap<Vec<usize>, usize> = HashMap::new();
    let possible_pattern_sizes = [9];

    // iterate over windows and record count of patterns
    for win_size in possible_pattern_sizes {
        trimmed_loads
            .windows(win_size)
            .map(|arr| arr.iter().map(|x| **x).collect_vec())
            .for_each(|v| *pattern_map.entry(v.to_owned()).or_default() += 1);
    }

    // identify most frequent pattern
    let max_counts: Vec<(&Vec<usize>, &usize)> = pattern_map.iter().max_set_by_key(|(_, v)| *v);
    let (pat, count) = max_counts
        .iter()
        .max_by(|(k1, _), (k2, _)| k1.len().cmp(&k2.len()))
        .expect("max by succeeds");

    println!("pattern {:?} occurs {} times", pat, count);

    let pat_start = trimmed_loads
        .iter()
        .position(|x| **x == pat[0])
        .expect("pattern starter")
        + trim_sample_start;

    println!("trim_sample_start: {}", trim_sample_start);
    println!("pat start: {}", pat_start);

    let desired_iteration: usize = 1000000000;
    let predicted_idx_of_desired_iteration = (desired_iteration - (pat_start + 1)) % pat.len();
    println!(
        "predicting iteration {} (trimmed to {}) to have north load of {}",
        desired_iteration,
        predicted_idx_of_desired_iteration,
        pat[predicted_idx_of_desired_iteration]
    );

    let mut load_freq_map: HashMap<usize, usize> = HashMap::new();
    north_loads.iter().enumerate().for_each(|(i, l)| {
        *load_freq_map.entry(l.to_owned()).or_default() += 1;
        //println!("{}:\t{}\t{}", i, l, load_freq_map.get(l).unwrap());
    });
}
