use crate::utils::FileUtils;
use anyhow::anyhow;

pub fn run() {
    let input_file = "src/puzzles/day_2/input.txt";
    let contents: Vec<String> = FileUtils::parse_text_file(input_file).unwrap();

    // Opponent: A for Rock, B for Paper, C for Scissors
    // Player: X for Rock, Y for Paper, Z for Scissors
    // round score = shape score + outcome score
    // shape score: 1 for Rock, 2 for Paper, 3 for Scissors
    // outcome score: 0 for lost, 3 for draw, 6 for winning

    let mut total_score: u32 = 0;
    for line in contents.iter() {
        let (opponent_choice, player_choice) = parse_line(&line).unwrap();

        let outcome = outcome(opponent_choice, player_choice);
        total_score += outcome.score() + player_choice.score();
    }
    dbg!(total_score);

    let mut total_score_part_2: u32 = 0;
    for line in contents.iter() {
        let (opponent_choice, player_expected_outcome) =
            parse_line::<PlayerExpectedOutcome>(&line).unwrap();

        let player_choice = player_choice(opponent_choice, player_expected_outcome);

        total_score_part_2 += player_expected_outcome.score() + player_choice.score();
    }
    dbg!(total_score_part_2);
}

enum OpponentChoice {
    A,
    B,
    C,
}

impl From<&str> for OpponentChoice {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            _ => {
                panic!("unknown opponent choice");
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PlayerChoice {
    X,
    Y,
    Z,
}

impl PlayerChoice {
    pub fn score(&self) -> u32 {
        match self {
            Self::X => 1,
            Self::Y => 2,
            Self::Z => 3,
        }
    }
}

impl From<&str> for PlayerChoice {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => {
                panic!("unknown player choice");
            }
        }
    }
}

#[derive(Clone, Copy)]
enum PlayerExpectedOutcome {
    X,
    Y,
    Z,
}

impl From<&str> for PlayerExpectedOutcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => {
                panic!("unknown player expected outcome");
            }
        }
    }
}

impl PlayerExpectedOutcome {
    pub fn score(&self) -> u32 {
        match self {
            Self::X => 0,
            Self::Y => 3,
            Self::Z => 6,
        }
    }
}

enum Outcome {
    Lost,
    Draw,
    Win,
}

impl Outcome {
    pub fn score(&self) -> u32 {
        match self {
            Self::Lost => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

fn parse_line<'a, T: From<&'a str>>(line: &'a str) -> anyhow::Result<(OpponentChoice, T)> {
    let mut line = line.trim().split_whitespace().take(2);

    let opponent_choice = if let Some(opponent_choice) = line.next() {
        OpponentChoice::from(opponent_choice)
    } else {
        return Err(anyhow!("invalid line, missing opponent choice"));
    };
    let player_choice = if let Some(player_choice) = line.next() {
        T::from(player_choice)
    } else {
        return Err(anyhow!("invalid line, missing player choice"));
    };

    Ok((opponent_choice, player_choice))
}

fn outcome(opponent_choice: OpponentChoice, player_choice: PlayerChoice) -> Outcome {
    match (opponent_choice, player_choice) {
        (OpponentChoice::A, PlayerChoice::X) => Outcome::Draw,
        (OpponentChoice::A, PlayerChoice::Y) => Outcome::Win,
        (OpponentChoice::A, PlayerChoice::Z) => Outcome::Lost,
        (OpponentChoice::B, PlayerChoice::X) => Outcome::Lost,
        (OpponentChoice::B, PlayerChoice::Y) => Outcome::Draw,
        (OpponentChoice::B, PlayerChoice::Z) => Outcome::Win,
        (OpponentChoice::C, PlayerChoice::X) => Outcome::Win,
        (OpponentChoice::C, PlayerChoice::Y) => Outcome::Lost,
        (OpponentChoice::C, PlayerChoice::Z) => Outcome::Draw,
    }
}

fn player_choice(
    opponent_choice: OpponentChoice,
    player_expected_outcome: PlayerExpectedOutcome,
) -> PlayerChoice {
    match (opponent_choice, player_expected_outcome) {
        (OpponentChoice::A, PlayerExpectedOutcome::X) => PlayerChoice::Z,
        (OpponentChoice::A, PlayerExpectedOutcome::Y) => PlayerChoice::X,
        (OpponentChoice::A, PlayerExpectedOutcome::Z) => PlayerChoice::Y,
        (OpponentChoice::B, PlayerExpectedOutcome::X) => PlayerChoice::X,
        (OpponentChoice::B, PlayerExpectedOutcome::Y) => PlayerChoice::Y,
        (OpponentChoice::B, PlayerExpectedOutcome::Z) => PlayerChoice::Z,
        (OpponentChoice::C, PlayerExpectedOutcome::X) => PlayerChoice::Y,
        (OpponentChoice::C, PlayerExpectedOutcome::Y) => PlayerChoice::Z,
        (OpponentChoice::C, PlayerExpectedOutcome::Z) => PlayerChoice::X,
    }
}
