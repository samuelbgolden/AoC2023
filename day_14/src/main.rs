use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{stdin, BufReader, Read},
    ops::Range,
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
        let mut section_start;
        let mut section_boulders: Vec<Point> = vec![];
        let mut boulder_count;

        let outer_range;
        let inner_range;

        match dir {
            Direction::North => {
                outer_range = 0..=(self.size.x - 1);
                inner_range = 0..=(self.size.y - 1);
            }
            Direction::East => {
                outer_range = (self.size.y - 1)..=0;
                inner_range = (self.size.x - 1)..=0;
            }
            Direction::South => {
                outer_range = (self.size.x - 1)..=0;
                inner_range = (self.size.y - 1)..=0;
            }
            Direction::West => {
                outer_range = 0..=(self.size.y - 1);
                inner_range = 0..=(self.size.x - 1);
            }
        }

        for outer in outer_range {
            section_start = 0;
            for inner in inner_range.clone() {
                let pnt;
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

                        // add boulders from the top down
                        for i in (section_start)..(section_start + boulder_count) {
                            let p;
                            if [Direction::North, Direction::South].contains(&dir) {
                                p = Point::new(outer, i);
                            } else {
                                p = Point::new(i, outer);
                            }
                            self.map.insert(p, Item::Boulder);
                        }

                        // reset boulder/wall info
                        assert!(section_boulders.is_empty());
                        section_start = inner + 1;
                    }
                    Item::Empty => (),
                }
            }

            boulder_count = section_boulders.len();
            for _ in 0..section_boulders.len() {
                self.map
                    .insert(section_boulders.pop().unwrap(), Item::Empty);
            }

            // add boulders from the top down
            for i in (section_start)..(section_start + boulder_count) {
                let p;
                if [Direction::North, Direction::South].contains(&dir) {
                    p = Point::new(outer, i);
                } else {
                    p = Point::new(i, outer);
                }
                self.map.insert(p, Item::Boulder);
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
    let iterations = 1;
    dish.roll_in_direction(Direction::East);
    println!("map rolled north:\n{}", dish);
    println!("load on north supports: {}", dish.calc_north_load());
}
