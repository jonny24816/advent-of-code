use std::fs;
use std::str::Split;
use phf::{phf_map};

static SCORES: phf::Map<&'static str, &'static i32> = phf_map! {

    "AX" => &4,
    "BX" => &1,
    "CX" => &7,
    "AY" => &8,
    "BY" => &5,
    "CY" => &2,
    "AZ" => &3,
    "BZ" => &9,
    "CZ" => &6,
};


fn main() {
    let games = split_per_game(r"C:\Users\jahug\git\advent-of-code\aoc2\input.txt");
    let mut sum = 0;
    for game in games {
        sum += check_game_score(&game);
    }
    println!("total score {}", sum);
}

fn split_per_game(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let games: Split<&str> = contents.split("\r\n");
    let mut games_output = Vec::new();
    for game in games {
        if game.to_string() == "" {
            println!("lol gotcha:)");
        } else {
            games_output.push(game.to_string());
        }
        
    }
    return games_output;
}

fn check_game_score(game: &str) -> Option<&&i32> {
    return SCORES.get(game).to;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_games() {
        let result = split_per_game(r"C:\Users\jahug\git\advent-of-code\aoc2\input.txt").len();
        assert_eq!(result, 2500);
    }
}
