use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    time::Duration,
};

const DISPLAY_DELAY: Duration = Duration::from_secs(2);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Tup2D {
    x: i32,
    y: i32,
}

impl Tup2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn new_doubled(&self) -> Self {
        Tup2D::new(self.x * 2, self.y * 2)
    }

    pub fn new_halved(&self) -> Self {
        Tup2D::new(self.x / 2, self.y / 2)
    }

    pub fn new_avg(&self, other: &Tup2D) -> Self {
        let new_y = match self.y == other.y {
            true => self.y,
            false => self.y.max(other.y) - 1,
        };
        let new_x = match self.x == other.x {
            true => self.x,
            false => self.x.max(other.x) - 1,
        };
        return Tup2D::new(new_x, new_y);
    }

    pub fn get_adj(&self) -> [Tup2D; 4] {
        [
            Tup2D::new(self.x - 1, self.y),
            Tup2D::new(self.x + 1, self.y),
            Tup2D::new(self.x, self.y + 1),
            Tup2D::new(self.x, self.y - 1),
        ]
    }
}

impl PartialOrd for Tup2D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.y.partial_cmp(&other.y) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.x.partial_cmp(&other.x)
    }
}

impl Ord for Tup2D {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.x.cmp(&other.x)
    }
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
enum TileType {
    Loop,
    Inside,
    Outside,
    Unknown,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                TileType::Loop => 'L',
                TileType::Inside => '.',
                TileType::Outside => '#',
                TileType::Unknown => 'U',
            }
        )
    }
}

pub struct Map {
    start: Option<Tup2D>,
    farthest_point: Option<Tup2D>,
    size: Tup2D,
    conns: HashMap<Tup2D, Vec<Tup2D>>,
    loop_points: HashSet<Tup2D>,
    start_conns_built: bool,
}

impl Map {
    pub fn new(size: Tup2D) -> Self {
        Self {
            start: None,
            farthest_point: None,
            size: size,
            conns: HashMap::new(),
            loop_points: HashSet::new(),
            start_conns_built: false,
        }
    }

    pub fn add_pos_from_char(&mut self, pos: Tup2D, chr: char) {
        assert!(self.check_in_bounds(pos));
        let conn: Vec<Tup2D> = match chr {
            '|' => vec![Tup2D::new(pos.x, pos.y - 1), Tup2D::new(pos.x, pos.y + 1)],
            '-' => vec![Tup2D::new(pos.x - 1, pos.y), Tup2D::new(pos.x + 1, pos.y)],
            'L' => vec![Tup2D::new(pos.x, pos.y - 1), Tup2D::new(pos.x + 1, pos.y)],
            'J' => vec![Tup2D::new(pos.x, pos.y - 1), Tup2D::new(pos.x - 1, pos.y)],
            '7' => vec![Tup2D::new(pos.x, pos.y + 1), Tup2D::new(pos.x - 1, pos.y)],
            'F' => vec![Tup2D::new(pos.x, pos.y + 1), Tup2D::new(pos.x + 1, pos.y)],
            '.' => vec![],
            'S' => {
                self.start = Some(pos);
                return;
            } // can't be sure of start conns here
            _ => panic!("unexpected char '{}'", chr),
        };
        self.conns.insert(
            pos,
            conn.iter()
                .filter(|c| self.check_in_bounds(**c))
                .map(|c| *c)
                .collect(),
        );
    }

    fn check_in_bounds(&self, pos: Tup2D) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.size.x && pos.y < self.size.y
    }

    fn get_connections(&self, pos: Tup2D) -> Option<Vec<Tup2D>> {
        self.conns.get(&pos).cloned()
    }

    // BFS style traversal staying in pipe
    pub fn find_dist_to_farthest_pos(&mut self) -> i32 {
        if !self.start_conns_built {
            self.build_start_conns();
        }
        let mut to_visit: HashMap<Tup2D, i32> = HashMap::new();
        let mut visited: HashSet<Tup2D> = HashSet::new();

        to_visit.insert(self.start.unwrap(), 0i32);

        let mut farthest_point: (Tup2D, i32) = (self.start.unwrap(), 0);

        while !to_visit.is_empty() {
            let mut potential_to_visit: Vec<(Tup2D, i32)> = vec![];
            for (pos, dist) in to_visit.iter_mut() {
                potential_to_visit.extend(
                    self.get_connections(*pos)
                        .unwrap()
                        .iter()
                        .map(|p| (*p, *dist + 1)),
                );
                visited.insert(*pos);
            }

            to_visit.clear();

            potential_to_visit
                .iter_mut()
                .filter(|(p, _)| !visited.contains(p))
                .for_each(|(p, d)| {
                    if *d > farthest_point.1 {
                        farthest_point = (*p, *d);
                    }
                    to_visit.insert(*p, *d);
                });
        }
        self.farthest_point = Some(farthest_point.0);
        println!("loop size: {}", visited.len());
        self.loop_points = visited;
        return farthest_point.1;
    }

    pub fn find_enclosed_positions(&mut self) -> i32 {
        if self.loop_points.is_empty() {
            self.find_dist_to_farthest_pos();
        }
        let doubled_size: Tup2D = self.size.new_doubled();
        let mut doubled_map: HashMap<Tup2D, TileType> = HashMap::new();

        // clean empty map
        (0..doubled_size.x)
            .cartesian_product(0..doubled_size.y)
            .for_each(|(x, y)| {
                doubled_map.insert(Tup2D::new(x, y), TileType::Unknown);
            });

        print_map_with_delay(&half_map(&doubled_map), std::time::Duration::from_secs(0));

        // double map size and interpolate points between connections
        let mut interpol_loop_points: Vec<Tup2D> = self
            .conns
            .iter()
            .filter(|(pos, _)| self.loop_points.contains(*pos))
            .flat_map(|(pos, conns)| {
                return conns.iter().map(|c| (*c, *pos));
            })
            .flat_map(|(p1, p2)| {
                let new_p1 = p1.new_doubled();
                let new_p2 = p2.new_doubled();
                let v = vec![new_p1, new_p1.new_avg(&new_p2), new_p2];
                v
            })
            .collect();
        interpol_loop_points.sort();
        interpol_loop_points.dedup();

        // set all remaining tiles to have type 'Loop'
        interpol_loop_points.iter().for_each(|p| {
            doubled_map.insert(*p, TileType::Loop);
        });

        print_map_with_delay(&half_map(&doubled_map), DISPLAY_DELAY);

        loop {
            match doubled_map.iter().find(|(_, t)| **t == TileType::Unknown) {
                Some((pos, _)) => {
                    let mut outside_known: bool = false;
                    let mut to_visit: Vec<Tup2D> = vec![*pos];
                    let mut visited: Vec<Tup2D> = vec![];

                    while !to_visit.is_empty() {
                        to_visit.iter().for_each(|p| visited.push(*p));
                        let mut new_to_visit: Vec<Tup2D> = vec![];
                        for tile in to_visit.iter() {
                            new_to_visit.extend(
                                tile.get_adj()
                                    .iter()
                                    .filter(|p| !visited.contains(p) && !new_to_visit.contains(p))
                                    .filter(|p| match doubled_map.get(&p) {
                                        Some(TileType::Outside) => {
                                            outside_known = true;
                                            return false;
                                        }
                                        Some(TileType::Loop) => false,
                                        Some(TileType::Inside) => panic!("how is this possible"),
                                        Some(TileType::Unknown) => true,
                                        None => {
                                            // out of bounds, means it must be outside the loop
                                            outside_known = true;
                                            return false;
                                        }
                                    })
                                    .collect::<Vec<_>>(),
                            );
                        }
                        new_to_visit.sort();
                        new_to_visit.dedup();

                        to_visit.clear();
                        to_visit.extend(new_to_visit);
                    }

                    visited.iter().for_each(|p| {
                        doubled_map.insert(
                            *p,
                            match outside_known {
                                true => TileType::Outside,
                                false => TileType::Inside,
                            },
                        );
                    });
                    print_map_with_delay(&half_map(&doubled_map), DISPLAY_DELAY);
                }
                None => break,
            }
        }

        return half_map(&doubled_map)
            .iter()
            .filter(|(_, t)| **t == TileType::Inside)
            .count() as i32;
    }

    fn build_start_conns(&mut self) {
        if let Some(s) = self.start {
            let neighbors: Vec<Tup2D> = vec![
                Tup2D::new(s.x - 1, s.y + 1),
                Tup2D::new(s.x - 1, s.y),
                Tup2D::new(s.x - 1, s.y - 1),
                Tup2D::new(s.x, s.y + 1),
                Tup2D::new(s.x, s.y - 1),
                Tup2D::new(s.x + 1, s.y + 1),
                Tup2D::new(s.x + 1, s.y),
                Tup2D::new(s.x + 1, s.y - 1),
            ];
            self.conns.insert(
                s,
                neighbors
                    .iter()
                    .filter(|p| self.check_in_bounds(**p))
                    .filter(|p| self.conns.get(*p).expect("neighbor pos").contains(&s))
                    .map(|p| *p)
                    .collect(),
            );
        } else {
            panic!("tried to build start connections without start pos set!");
        }
    }
}

fn half_map<T: Copy>(map: &HashMap<Tup2D, T>) -> HashMap<Tup2D, T> {
    let mut new_map: HashMap<Tup2D, T> = HashMap::new();
    map.iter().for_each(|(p, t)| {
        if (p.x % 2 == 0) && (p.y % 2 == 0) {
            new_map.insert(p.new_halved(), *t);
        }
    });
    new_map
}

fn print_map_with_delay<T: Display + Copy>(map: &HashMap<Tup2D, T>, delay: Duration) {
    let mut size_x: i32 = 0;
    map.keys().for_each(|p| {
        size_x = size_x.max(p.x);
    });
    size_x += 1;

    std::thread::sleep(delay);
    clear_term();
    println!(
        "{}\n",
        map.iter()
            .sorted_by_key(|(p, _)| *p)
            .chunks(size_x as usize)
            .into_iter()
            .map(|chunk| chunk.into_iter().map(|(_, t)| *t).join(" "))
            .join("\n")
    );
}

fn clear_term() {
    print!("{esc}c", esc = 27 as char);
}
