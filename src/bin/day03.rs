use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input() -> Result<Vec<(String, String)>> {
    let input = File::open("input/day03.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| {
            let line = line?;
            let (first, second) = line.split_at(line.len() / 2);
            Ok((first.to_owned(), second.to_owned()))
        })
        .collect()
}

fn char_to_priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - b'a' as u32 + 1,
        'A'..='Z' => c as u32 - b'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn main() -> Result<()> {
    let data = read_input()?;

    let part1 = data
        .iter()
        .map(|(first, second)| {
            // Convert to set of chars.
            let first = first.chars().collect::<HashSet<_>>();
            let second = second.chars().collect::<HashSet<_>>();

            let mut intersection = first.intersection(&second);
            let common = intersection.next();

            assert!(intersection.next() == None);

            let char = common.context("no common letters")?;
            Ok(char_to_priority(*char))
        })
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum::<u32>();

    println!("part1: {}", part1);

    let part2 = data
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            chunk
                .map(|(first, second)| {
                    let combined = format!("{}{}", first, second);
                    combined.chars().collect::<HashSet<_>>()
                })
                .reduce(|a, b| a.intersection(&b).copied().collect())
        })
        .map(|intersection| {
            let mut chars = intersection.context("fold failed")?.into_iter();

            let common = chars.next();

            assert!(chars.next() == None);

            let char = common.context("no common letters")?;
            Ok(char_to_priority(char))
        })
        .collect::<Result<Vec<u32>>>()?
        .iter()
        .sum::<u32>();

    println!("part2: {}", part2);

    Ok(())
}
