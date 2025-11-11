use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let file1 = OpenOptions::new().read(true).open("input/input.txt")?;
    let file2 = OpenOptions::new().read(true).open("input/input.txt")?;
    let buf1 = BufReader::new(file1);
    let buf2 = BufReader::new(file2);
    part1(buf1)?;
    part2(buf2)?;
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

fn part2(buf: BufReader<File>) -> std::io::Result<()> {
    let mut hs = HashSet::<i32>::new();
    let mut freq = 0i32;
    hs.insert(freq);
    let vec: Vec<i32> = buf
        .lines()
        .map(|item| item.unwrap().replace("+", "").parse::<i32>().unwrap())
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
