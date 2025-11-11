use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let input_file = OpenOptions::new()
        .read(true)
        .open("input/input.txt")
        .expect("file exists and is readable");
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    let buf_reader = BufReader::new(input_file);
    let mut pair: Vec<&str> = Vec::with_capacity(2);
    for line in buf_reader.lines() {
        let line = line?;
        pair = line.split_whitespace().collect();
        left_list.push(pair[0].parse::<i32>().unwrap());
        right_list.push(pair[1].parse::<i32>().unwrap());
    }
    // println!("{:#?}", left_list);
    left_list.sort();
    right_list.sort();
    let distance = left_list
        .iter()
        .zip(right_list)
        .fold(0, |acc, (left, right)| acc + (left - right).abs());
    println!("distance={distance}");
    Ok(())
}
