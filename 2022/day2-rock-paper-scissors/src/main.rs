use std::io;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(FromPrimitive, ToPrimitive, PartialEq, Eq, Debug)]
enum Shape {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

#[derive(PartialEq, ToPrimitive, FromPrimitive, Eq, Debug)]
enum MatchResult {
    Win = 2,
    Tie = 1,
    Loss = 0,
}

fn calculate_move(opponent_move: &Shape, required_result: &MatchResult) -> Option<Shape> {
    let ppv = ToPrimitive::to_i32(opponent_move)?;
    let cpv = ToPrimitive::to_i32(required_result)?;

    println!("Calculated Shape: {}", (2 + ppv + cpv) % 3);
    FromPrimitive::from_i32((2 + ppv + cpv) % 3)
}

fn calculate_win(perspective_player: &Shape, challenged_player: &Shape) -> Option<MatchResult> {
    let ppv = ToPrimitive::to_i32(perspective_player)?;
    let cpv = ToPrimitive::to_i32(challenged_player)?;

    let result = (3 + ppv - cpv) % 3;

    FromPrimitive::from_i32((result + 1) % 3)

    //if result == 0 {
    //    Some(MatchResult::Tie)
    //} else if result == 1 {
    //    Some(MatchResult::Win)
    //} else {
    //    Some(MatchResult::Loss)
    //}
}

#[derive(Debug)]
enum InputVals {
    OpponentChoice(Shape),
    RequiredResult(MatchResult),
}

fn main() -> std::io::Result<()> {
    let mut input_buf = String::new();

    let mut total = 0;

    while match std::io::stdin().read_line(&mut input_buf) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("Error while reading line: {:?}", error);
            return Err(error);
        }
    } != 0
    {
        let shapes: Vec<InputVals> = match input_buf
            .split_whitespace()
            .into_iter()
            .map(|s| {
                if (s.as_bytes()[0] - 'A' as u8) <= 3 {
                    FromPrimitive::from_u8(s.as_bytes()[0] - 'A' as u8)
                        .map(|val| InputVals::OpponentChoice(val))
                } else {
                    FromPrimitive::from_u8(s.as_bytes()[0] - 'X' as u8)
                        .map(|val| InputVals::RequiredResult(val))
                }
            })
            .collect()
        {
            Some(numbers) => numbers,
            None => {
                eprintln!("Error while parsing shapes");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Unable to parse as shape",
                ));
            }
        };

        input_buf.clear();

        if shapes.len() != 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Expected 2 numbers, got: {}", shapes.len()),
            ));
        }

        let opponent_choice = match &shapes[0] {
            InputVals::OpponentChoice(opch) => opch,
            InputVals::RequiredResult(val) => {
                eprintln!("Opponent choice is a RequiredResult");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Oppenent choice is a RequiredResult {:?}", val),
                ));
            }
        };

        let required_result = match &shapes[1] {
            InputVals::RequiredResult(res) => res,
            InputVals::OpponentChoice(val) => {
                eprintln!("Required result is an OpponentChoice");
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Required result is an OpponentChoice {:?}", val),
                ));
            }
        };

        let player_move = match calculate_move(&opponent_choice, &required_result) {
            Some(res) => res,
            None => {
                eprintln!(
                    "Error while calculating match move {:?}, {:?}",
                    opponent_choice, required_result
                );
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Error while calculating match move {:?}, {:?}",
                        opponent_choice, required_result
                    ),
                ));
            }
        };

        total += match player_move {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        total += match required_result {
            MatchResult::Win => 6,
            MatchResult::Tie => 3,
            MatchResult::Loss => 0,
        };

        println!(
            "Match: Op: {:?} Res:{:?} -> Mov: {:?}",
            opponent_choice, required_result, &player_move
        );
    }

    println!("Total points: {}", total);

    Ok(())
}
