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

    fn roll_north(&mut self) {
        let mut section_start;
        let mut section_boulders: Vec<Point> = vec![];
        let mut boulder_count;
        for col in 0..self.size.x {
            section_start = 0;
            //println!("rolling col {}", col);
            for y in 0..self.size.y {
                let pnt = Point::new(col, y);
                match self.get(&pnt) {
                    Item::Boulder => section_boulders.push(pnt),
                    Item::Wall => {
                        //println!(
                        //    "\twall encountered @ {}: boulders {:?}, sect start {}",
                        //    pnt.y, section_boulders, section_start
                        //);
                        boulder_count = section_boulders.len();

                        // clear previous boulder positions
                        for _ in 0..boulder_count {
                            self.map
                                .insert(section_boulders.pop().unwrap(), Item::Empty);
                        }

                        // add boulders from the top down
                        for inner_y in (section_start)..(section_start + boulder_count) {
                            self.map.insert(Point::new(col, inner_y), Item::Boulder);
                        }

                        // reset boulder/wall info
                        assert!(section_boulders.is_empty());
                        section_start = y + 1;
                    }
                    Item::Empty => (),
                }
            }

            //println!(
            //    "\tend of col: boulders {:?}, sect start {}",
            //    section_boulders, section_start
            //);

            boulder_count = section_boulders.len();
            for _ in 0..section_boulders.len() {
                self.map
                    .insert(section_boulders.pop().unwrap(), Item::Empty);
            }

            // add boulders from the top down
            for inner_y in (section_start)..(section_start + boulder_count) {
                self.map.insert(Point::new(col, inner_y), Item::Boulder);
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
    dish.roll_north();
    println!("map rolled north:\n{}", dish);
    println!("load on north supports: {}", dish.calc_north_load());
}
