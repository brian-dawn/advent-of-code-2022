use anyhow::Result;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn read_input() -> Result<Vec<Option<i32>>> {
    let input = File::open("input/day01.txt")?;
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .map(|line| Ok(line?.parse::<i32>().ok()))
        .collect()
}

fn main() -> Result<()> {
    let elves = read_input()?
        .into_iter()
        .group_by(|x| x.is_some())
        .into_iter()
        .filter_map(|(_, group)| group.collect::<Option<Vec<_>>>())
        .map(|group| group.into_iter().sum::<i32>())
        .sorted()
        .rev();

    let top_three = elves.take(3).collect::<Vec<_>>();
    let part1 = top_three[0];
    let part2 = top_three.iter().sum::<i32>();

    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
