use std::fs::File;
use std::io::Read;

fn main() {
    let params = (12, 13, 14);
    let mut data = String::new();
    let mut f = File::open("test_input.txt").expect("should read file");
    f.read_to_string(&mut data).expect("should read data");
    let game_id_sum = data.split("\n").into_iter().fold(0, |acc, g| {
        let game = Game::from(g.to_string());
        match game.is_valid(params.0, params.1, params.2) {
            true => game.id + acc,
            false => acc,
        }
    });
    println!("{}", game_id_sum)
}

// 12 red, 13 green, 14 blue

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, red: i32, green: i32, blue: i32) -> bool {
        for r in &self.rounds {
            if !r.is_valid(red, green, blue) {
                return false;
            }
        }
        true
    }
}

impl From<String> for Game {
    fn from(value: String) -> Self {
        let parts: Vec<&str> = value.split(":").collect();
        let game_id = parts
            .get(0)
            .expect("couldn't get game id")
            .replace("Game ", "")
            .parse()
            .expect("should be an i32");
        let rounds: Vec<Round> = parts
            .get(1)
            .expect("couldn't get rounds")
            .split(";")
            .into_iter()
            .map(|r| Round::from(r.to_string()))
            .collect();
        Self {
            id: game_id,
            rounds,
        }
    }
}

#[derive(Debug, Default)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn is_valid(&self, red: i32, green: i32, blue: i32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}
// 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

impl From<String> for Round {
    fn from(value: String) -> Self {
        let results: Vec<&str> = value.split(",").collect();
        let mut round = Round::default();
        for r in results {
            let parts: Vec<&str> = r.trim().split(" ").collect();
            let count: i32 = parts[0].parse().expect("should be able to convert to i32");
            match parts[1].to_lowercase().as_str() {
                "red" => round.red = count,
                "green" => round.green = count,
                "blue" => round.blue = count,
                _ => panic!("unexpected color"),
            }
        }
        round
    }
}
