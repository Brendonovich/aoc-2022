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
    fn from_str(string: &str) -> Option<Self> {
        let mut data = string.split(" ").filter_map(Play::from_str);

        Some(Self {
            first_play: data.next()?,
            second_play: data.next()?,
        })
    }

    fn with_result(first_play: Play, result: GameResult) -> Self {
        Self {
            second_play: first_play.next_for_result(&result),
            first_play,
        }
    }

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
    let input = include_str!("./input.txt");

    println!(
        "Part A Score: {}",
        input
            .lines()
            .flat_map(|line| Game::from_str(line))
            .map(Game::evaluate)
            .sum::<i32>()
    );

    println!(
        "Part B Score: {}",
        input
            .lines()
            .flat_map(|line| {
                let mut chars = line.split(" ");

                Some(Game::with_result(
                    chars.next().and_then(Play::from_str)?,
                    chars.next().and_then(GameResult::from_str)?,
                ))
            })
            .map(Game::evaluate)
            .sum::<i32>()
    );
}
