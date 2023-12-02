use std::io::{stdin, BufRead, BufReader};

#[derive(Debug)]
struct Game {
    id: u32,
    r: Vec<u32>,
    g: Vec<u32>,
    b: Vec<u32>,
}

fn main() {
    let lines = BufReader::new(stdin().lock()).lines();
    let games: Vec<Game> = lines
        .filter_map(|l| match l {
            Ok(s) => Some(parse_line(s)),
            Err(_) => None,
        })
        .collect();

    let answer: u32 = games
        .iter()
        .map(|game| {
            game.r.iter().max().unwrap_or(&0)
                * game.g.iter().max().unwrap_or(&0)
                * game.b.iter().max().unwrap_or(&0)
        })
        .sum();

    println!("{}", answer);
}

fn parse_line(line: String) -> Game {
    let mut game = Game {
        id: 0u32,
        r: vec![],
        g: vec![],
        b: vec![],
    };

    let mut iter = line.split(':');

    // get game id
    game.id = match iter.next() {
        Some(s) => s.replace("Game ", "").parse::<u32>().unwrap(),
        None => panic!(),
    };

    // get cube counts
    iter.next()
        .unwrap_or_default()
        .split(';')
        .for_each(|round| {
            // get the colors in each round
            round.split(',').map(str::trim).for_each(|color| {
                // separate num and color
                let mut inner_iter = color.split(' ');
                let num = inner_iter.next().unwrap().parse::<u32>().unwrap();
                match inner_iter.next().unwrap() {
                    "red" => game.r.push(num),
                    "green" => game.g.push(num),
                    "blue" => game.b.push(num),
                    &_ => panic!(),
                }
            });
        });

    return game;
}
