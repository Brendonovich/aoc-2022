#[derive(Clone)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Play {
    fn from_str(string: &str) -> Option<Self> {
        Some(match string {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => return None,
        })
    }

    fn next_for_result(&self, result: &GameResult) -> Self {
        use GameResult::*;
        use Play::*;

        match (result, self) {
            (Win, Rock) => Paper,
            (Win, Paper) => Scissors,
            (Win, Scissors) => Rock,
            (Lose, Rock) => Scissors,
            (Lose, Paper) => Rock,
            (Lose, Scissors) => Paper,
            (Draw, p) => p.clone(),
        }
    }
}

enum GameResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl GameResult {
    fn from_str(string: &str) -> Option<Self> {
        use GameResult::*;

        Some(match string {
            "X" => Lose,
            "Y" => Draw,
            "Z" => Win,
            _ => return None,
        })
    }
}

struct Game {
    pub first_play: Play,
    pub second_play: Play,
}

impl Game {
    fn evaluate(self) -> i32 {
        use GameResult::*;
        use Play::*;

        let result = match (&self.first_play, &self.second_play) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            _ => Lose,
        };

        result as i32 + self.second_play as i32
    }
}

fn main() {
    let lines = include_str!("./input.txt")
        .lines()
        .flat_map(|line| line.split_once(" "));

    println!(
        "Part A Score: {}",
        lines
            .clone()
            .flat_map(|(first_play, second_play)| {
                Some(Game {
                    first_play: Play::from_str(first_play)?,
                    second_play: Play::from_str(second_play)?,
                })
            })
            .map(Game::evaluate)
            .sum::<i32>()
    );

    println!(
        "Part B Score: {}",
        lines
            .flat_map(|(first_play, desired_result)| {
                let first_play = Play::from_str(first_play)?;
                let desired_result = GameResult::from_str(desired_result)?;

                Some(Game {
                    second_play: first_play.next_for_result(&desired_result),
                    first_play,
                })
            })
            .map(Game::evaluate)
            .sum::<i32>()
    );
}
