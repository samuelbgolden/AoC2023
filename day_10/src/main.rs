use std::io::{stdin, BufRead, BufReader};

use map::{Map, Tup2D};
mod map;

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let data = lines.filter_map(Result::ok).collect::<Vec<String>>();
    let mut map: Map = Map::new(Tup2D::new(data[0].len() as i32, data.len() as i32));

    for (y, line) in data.iter().enumerate() {
        for (x, chr) in line.char_indices() {
            map.add_pos_from_char(Tup2D::new(x as i32, y as i32), chr);
        }
    }

    println!("enclosed tiles: {}", map.find_enclosed_positions());
}
