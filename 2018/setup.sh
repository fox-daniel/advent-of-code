#!/usr/bin/env zsh

dir_name="aoc$1"
file_name="main.rs"

cargo new "$dir_name"

mkdir "$dir_name/input"

main_content="use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> std::io::Result<()> {
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    Ok(())
}" 

echo "$main_content" > "$dir_name/main.rs"

