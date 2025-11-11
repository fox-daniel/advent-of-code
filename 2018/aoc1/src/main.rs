use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let file = OpenOptions::new().read(true).open("input/input.txt")?;
    let buf = BufReader::new(file);
    part1(buf)?;
    Ok(())
}

fn part1(buf: BufReader<File>) -> std::io::Result<()> {
    let freq = buf.lines().fold(0, |acc, s| {
        acc + s.unwrap().replace("+", "").parse::<i32>().unwrap()
    });
    let freq = format!("{freq}\n");
    let freq = freq.as_bytes();
    std::io::stdout().write_all(freq)?;
    Ok(())
}

// fn part2(file: File) -> std::io::Result<()> {}
