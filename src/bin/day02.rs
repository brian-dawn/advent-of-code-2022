use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn from_choice(choice: &Choice) -> Self {
        match choice {
            Choice::Rock => Outcome::Lose,
            Choice::Paper => Outcome::Draw,
            Choice::Scissors => Outcome::Win,
        }
    }
}

fn parse_line(line: &str) -> Result<(Choice, Choice)> {
    let mut parts = line.split(" ");
    let choice = match parts.next() {
        Some("A") => Choice::Rock,
        Some("B") => Choice::Paper,
        Some("C") => Choice::Scissors,
        _ => return Err(anyhow!("invalid choice")),
    };

    let response = match parts.next() {
        Some("X") => Choice::Rock,
        Some("Y") => Choice::Paper,
        Some("Z") => Choice::Scissors,
        _ => return Err(anyhow!("invalid choice")),
    };

    Ok((choice, response))
}

fn read_input() -> Result<Vec<(Choice, Choice)>> {
    let input = File::open("input/day02.txt")?;
    let buffered = BufReader::new(input);

    buffered.lines().map(|line| parse_line(&line?)).collect()
}

fn score(choice: &Choice, response: &Choice) -> u32 {
    let win = 6;
    let draw = 3;

    let rock = 1;
    let paper = 2;
    let scissors = 3;

    match (choice, response) {
        (Choice::Rock, Choice::Scissors) => scissors,
        (Choice::Paper, Choice::Rock) => rock,
        (Choice::Scissors, Choice::Paper) => paper,
        (Choice::Rock, Choice::Paper) => paper + win,
        (Choice::Paper, Choice::Scissors) => scissors + win,
        (Choice::Scissors, Choice::Rock) => rock + win,
        (Choice::Rock, Choice::Rock) => rock + draw,
        (Choice::Paper, Choice::Paper) => paper + draw,
        (Choice::Scissors, Choice::Scissors) => scissors + draw,
    }
}

fn main() -> Result<()> {
    let data = read_input()?;

    let part1 = data
        .iter()
        .map(|(choice, response)| score(choice, response))
        .sum::<u32>();

    let part2 = data
        .into_iter()
        .map(|(choice, response)| (choice, Outcome::from_choice(&response)))
        .map(|(choice, outcome)| {
            let resp = match (&choice, outcome) {
                (Choice::Rock, Outcome::Win) => Choice::Paper,
                (Choice::Paper, Outcome::Win) => Choice::Scissors,
                (Choice::Scissors, Outcome::Win) => Choice::Rock,
                (Choice::Rock, Outcome::Lose) => Choice::Scissors,
                (Choice::Paper, Outcome::Lose) => Choice::Rock,
                (Choice::Scissors, Outcome::Lose) => Choice::Paper,
                (choice, Outcome::Draw) => choice.clone(),
            };
            (choice, resp)
        })
        .map(|(choice, response)| score(&choice, &response))
        .sum::<u32>();

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
