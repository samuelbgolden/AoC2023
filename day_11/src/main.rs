use itertools::Itertools;
use std::io::{stdin, BufRead, BufReader};

static EXPANSION_DIST: usize = 1000000;

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();

    let mut size_of_the_universe: (usize, usize) = (0, 0);

    // iterate over the input, when a '#' is encountered, mark it's xy pos
    let mut galaxy_positions: (Vec<usize>, Vec<usize>) = (vec![], vec![]);
    for line in lines.enumerate() {
        if let (y, Ok(data)) = line {
            size_of_the_universe.1 += 1;
            for (x, chr) in data.char_indices() {
                size_of_the_universe.0 += 1;
                match chr {
                    '#' => {
                        galaxy_positions.0.push(x);
                        galaxy_positions.1.push(y);
                    }
                    _ => continue,
                }
            }
        }
    }

    // iterate over galaxy positions, update it's position for the expanded universe by
    //   figuring out how many rows/cols have no xy pos and adding the count less to the coord
    let expanded_rows: Vec<usize> = (0..size_of_the_universe.0)
        .filter(|x| !galaxy_positions.1.contains(x))
        .collect();
    let expanded_cols: Vec<usize> = (0..size_of_the_universe.1)
        .filter(|y| !galaxy_positions.0.contains(y))
        .collect();
    galaxy_positions.0.iter_mut().for_each(|x| {
        *x += (expanded_cols.iter().filter(|col| *col < x).count())
            * 1usize.max(EXPANSION_DIST - 1usize)
    });
    galaxy_positions.1.iter_mut().for_each(|y| {
        *y += (expanded_rows.iter().filter(|row| *row < y).count())
            * 1usize.max(EXPANSION_DIST - 1usize)
    });

    let dist_sum: usize = galaxy_positions
        .0
        .iter()
        .zip(galaxy_positions.1.iter())
        .into_iter()
        .combinations(2)
        .map(|gs| {
            let g1 = gs[0];
            let g2 = gs[1];
            let dx = g1.0.max(g2.0) - g1.0.min(g2.0);
            let dy = g1.1.max(g2.1) - g1.1.min(g2.1);
            dx + dy
        })
        .sum();

    println!("sum of shortest paths: {}", dist_sum);
}
