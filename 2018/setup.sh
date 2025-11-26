#!/usr/bin/env zsh

dir_name="aoc$1"
file_name="main.rs"

cargo new "$dir_name"

mkdir "$dir_name/input"

session_cookie="$AOC_SESSION_COOKIE"

curl -b "session=$session_cookie" "https://adventofcode.com/2018/day/$1/input" > "$dir_name/input/input.txt"

main_content='use std::fs;
use std::io::Write;


fn main() -> Result<(), Box<dyn std::error::Error>> {
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
}'

echo "$main_content" > "$dir_name/src/main.rs"

