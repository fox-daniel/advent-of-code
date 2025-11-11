use std::collections::HashSet;
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> std::io::Result<()> {
    let freq = input
        .lines()
        .fold(0, |acc, s| acc + s.replace("+", "").parse::<i32>().unwrap());
    let freq = format!("{freq}\n");
    let freq = freq.as_bytes();
    std::io::stdout().write_all(freq)?;
    Ok(())
}

fn part2(input: &str) -> std::io::Result<()> {
    let mut hs = HashSet::<i32>::new();
    let mut freq = 0i32;
    hs.insert(freq);
    let vec: Vec<i32> = input
        .lines()
        .map(|item| item.replace("+", "").parse::<i32>().unwrap())
        .collect();
    for item in vec.iter().cycle() {
        freq += item;
        if !hs.insert(freq) {
            println!("{freq}");
            break;
        }
    }
    Ok(())
}
