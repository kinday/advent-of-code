const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const LOSS: i32 = 0;
const DRAW: i32 = 3;
const WIN: i32 = 6;

enum ChoiceScore {
    ROCK,
    PAPER,
    SCISSORS,
}

struct Round(ChoiceScore, ChoiceScore);

fn parse_input(lines: Vec<String>) -> Vec<Round> {
    let mut result = Vec::new();

    for line in lines {
        let choices: Vec<&str> = line.split(" ").collect();
        let opponent_choice = match choices.first() {
            Some(&"A") => ChoiceScore::ROCK,
            Some(&"B") => ChoiceScore::PAPER,
            Some(&"C") => ChoiceScore::SCISSORS,
            Some(&&_) => panic! {"wat"},
            None => panic! {"oops"},
        };
        let player_choice = match choices.last() {
            Some(&"X") => ChoiceScore::ROCK,
            Some(&"Y") => ChoiceScore::PAPER,
            Some(&"Z") => ChoiceScore::SCISSORS,
            Some(&&_) => panic! {"wat"},
            None => panic! {"oops"},
        };
        result.push(Round(opponent_choice, player_choice))
    }

    return result;
}

fn solve_first(rounds: &Vec<Round>) -> String {
    let mut score: i32 = 0;
    for round in rounds {
        let round_score: i32 = match round {
            Round(ChoiceScore::ROCK, choice) => match choice {
                ChoiceScore::ROCK => ROCK + DRAW,
                ChoiceScore::PAPER => PAPER + WIN,
                ChoiceScore::SCISSORS => SCISSORS + LOSS,
            },
            Round(ChoiceScore::PAPER, choice) => match choice {
                ChoiceScore::ROCK => ROCK + LOSS,
                ChoiceScore::PAPER => PAPER + DRAW,
                ChoiceScore::SCISSORS => SCISSORS + WIN,
            },
            Round(ChoiceScore::SCISSORS, choice) => match choice {
                ChoiceScore::ROCK => ROCK + WIN,
                ChoiceScore::PAPER => PAPER + LOSS,
                ChoiceScore::SCISSORS => SCISSORS + DRAW,
            },
        };
        score += round_score
    }
    return score.to_string();
}

fn solve_second(rounds: &Vec<Round>) -> String {
    let mut score: i32 = 0;
    for round in rounds {
        let desired_move = match &round {
            // Lose
            Round(opponent_move, ChoiceScore::ROCK) => match opponent_move {
                ChoiceScore::ROCK => ChoiceScore::SCISSORS,
                ChoiceScore::PAPER => ChoiceScore::ROCK,
                ChoiceScore::SCISSORS => ChoiceScore::PAPER,
            },
            // Draw
            Round(opponent_move, ChoiceScore::PAPER) => match opponent_move {
                ChoiceScore::ROCK => ChoiceScore::ROCK,
                ChoiceScore::PAPER => ChoiceScore::PAPER,
                ChoiceScore::SCISSORS => ChoiceScore::SCISSORS,
            },
            // Win
            Round(opponent_move, ChoiceScore::SCISSORS) => match opponent_move {
                ChoiceScore::ROCK => ChoiceScore::PAPER,
                ChoiceScore::PAPER => ChoiceScore::SCISSORS,
                ChoiceScore::SCISSORS => ChoiceScore::ROCK,
            },
        };
        let round_score: i32 = match round {
            Round(ChoiceScore::ROCK, _) => match desired_move {
                ChoiceScore::ROCK => ROCK + DRAW,
                ChoiceScore::PAPER => PAPER + WIN,
                ChoiceScore::SCISSORS => SCISSORS + LOSS,
            },
            Round(ChoiceScore::PAPER, _) => match desired_move {
                ChoiceScore::ROCK => ROCK + LOSS,
                ChoiceScore::PAPER => PAPER + DRAW,
                ChoiceScore::SCISSORS => SCISSORS + WIN,
            },
            Round(ChoiceScore::SCISSORS, _) => match desired_move {
                ChoiceScore::ROCK => ROCK + WIN,
                ChoiceScore::PAPER => PAPER + LOSS,
                ChoiceScore::SCISSORS => SCISSORS + DRAW,
            },
        };
        score += round_score
    }
    return score.to_string();
}

pub fn solve(input: Vec<String>) -> (String, String) {
    let data = parse_input(input);
    return (solve_first(&data), solve_second(&data));
}
