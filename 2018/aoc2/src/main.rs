use std::collections::HashMap;
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    // part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> std::io::Result<()> {
    let mut doubles: u32 = 0;
    let mut triples: u32 = 0;
    let mut counts = HashMap::<char, u32>::new();
    for line in input.lines() {
        counts.clear();
        for char in line.chars() {
            counts
                .entry(char)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let (has_double, has_triple) = counts.values().fold((false, false), |acc, v| {
            if *v == 2 {
                (true, acc.1)
            } else if *v == 3 {
                (acc.0, true)
            } else {
                acc
            }
        });
        if has_double {
            doubles += 1;
        };
        if has_triple {
            triples += 1;
        };
    }
    let check_sum = doubles * triples;
    let check_sum = format!("{check_sum}\n");
    std::io::stdout().write_all(check_sum.as_bytes())?;
    Ok(())
}

// fn part2(input: &str) -> std::io::Result<()> {
//     let mut hs = HashSet::<i32>::new();
//     let mut freq = 0i32;
//     hs.insert(freq);
//     let vec: Vec<i32> = input
//         .lines()
//         .map(|item| item.replace("+", "").parse::<i32>().unwrap())
//         .collect();
//     for item in vec.iter().cycle() {
//         freq += item;
//         if !hs.insert(freq) {
//             println!("{freq}");
//             break;
//         }
//     }
//     Ok(())
// }
