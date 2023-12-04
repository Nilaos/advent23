use aoc_driver::*;
use serde::Deserialize;

#[derive(Debug)]
struct Show {
    red: i32,
    green: i32,
    blue: i32,
}

impl Show {
    fn new(red: i32, green: i32, blue: i32) -> Show {
        Show { red, green, blue }
    }

    fn from_str(input: &str) -> Show {
        let iter = input.trim().split(',');
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        // println!("{:?}", iter);
        for i in iter {
            let mut iter2 = i.trim().split_whitespace();
            let value = iter2.next().unwrap().parse::<i32>().unwrap();
            let color = iter2.next().unwrap();
            match color {
                "red" => red = value,
                "green" => green = value,
                "blue" => blue = value,
                _ => panic!("Unknown color"),
            }
        }
        Show::new(red, green, blue)
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    plays: Vec<Show>,
}

impl Game {
    fn new(id: i32, plays: Vec<Show>) -> Game {
        Game { id, plays }
    }

    fn from_str(input: &str) -> Game {
        let trim = input.trim_start_matches("Game").trim_start();
        let id = trim
            .split(":")
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap_or_default();
        let games = trim
            .split(":")
            .skip(1)
            .next()
            .unwrap_or_default()
            .split(';');

        // println!("ID {}: {:?}", id, games);

        let mut plays = Vec::new();

        for game in games {
            let record = Show::from_str(game);
            plays.push(record);
        }
        Game::new(id, plays)
    }
}

fn part_1(input: &str) -> String {
    let red_t = 12;
    let green_t = 13;
    let blue_t = 14;

    let mut id_sum = 0;
    for line in input.split('\n') {
        let game = Game::from_str(line);
        let mut valid = true;
        for play in &game.plays {
            if play.red > red_t || play.green > green_t || play.blue > blue_t {
                println!("Invalid play: {:?} in game {}", play, game.id);
                valid = false;
                break;
            }
        }
        if !valid {
            println!("Invalid game: {:?}", game);
            continue;
        } else {
            id_sum += game.id;
            println!("Valid game: {:?}, id_sum {}", game, id_sum);
        }
    }
    println!("id_sum: {}", id_sum);
    // unimplemented!("Answer for part 1");
    id_sum.to_string()
}

fn part_2(input: &str) -> String {
    let mut id_sum = 0;
    for line in input.split('\n') {
        let game = Game::from_str(line);
        let mut valid = true;

        let mut red_t = 0;
        let mut green_t = 0;
        let mut blue_t = 0;
        for play in &game.plays {
            if play.red > red_t {
                red_t = play.red;
            }
            if play.green > green_t {
                green_t = play.green;
            }
            if play.blue > blue_t {
                blue_t = play.blue;
            }
        }
        id_sum += blue_t * green_t * red_t;
    }
    println!("id_sum: {}", id_sum);
    // unimplemented!("Answer for part 2");
    id_sum.to_string()
}

fn main() {
    let session = std::fs::read_to_string("../.session.txt").unwrap();
    aoc_magic!(session.as_str(), 2023:2:2, part_2).unwrap();
    println!("Well done! 🦀🦀🦀")
}
