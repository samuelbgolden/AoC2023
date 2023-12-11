use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Tup2D {
    x: i32,
    y: i32,
}

impl Tup2D {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct Map {
    start: Option<Tup2D>,
    size: Tup2D,
    conns: HashMap<Tup2D, Vec<Tup2D>>,
    start_conns_built: bool,
}

impl Map {
    pub fn new(size: Tup2D) -> Self {
        Self {
            start: None,
            size: size,
            conns: HashMap::new(),
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

    // BFS style traversal to gaurantee shortest path
    pub fn find_dist_to_farthest_pos(&mut self) -> i32 {
        if !self.start_conns_built {
            self.build_start_conns();
        }
        let mut to_visit: HashMap<Tup2D, i32> = HashMap::new();
        let mut visited: HashSet<Tup2D> = HashSet::new();

        to_visit.insert(self.start.unwrap(), 0i32);

        let mut max: i32 = 0;

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
                    if *d > max {
                        max = *d;
                    }
                    to_visit.insert(*p, *d);
                });
        }
        return max;
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
