use std::cmp::min;
use std::fs;
use std::io::Write;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/input.txt")?;
    let input = input.trim();
    part1(input)?;
    part2(input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    writeln!(std::io::stdout(), "length: {}", part1_result(input).len()).ok();
    Ok(())
}

fn part1_result(s: &str) -> Vec<u8> {
    let mut protein: Vec<u8> = s.as_bytes().to_vec();
    react(&mut protein)
}

fn react(protein: &mut [u8]) -> Vec<u8> {
    let mut prefix: Vec<u8> = vec![];
    let len = protein.len();
    let mut idx = 0;
    while idx < len {
        let mut c1 = protein[idx];
        if prefix.is_empty() {
            prefix.push(c1);
            idx += 1;
            if idx < len {
                c1 = protein[idx];
            } else {
                break;
            }
        }
        if annihilate(c1, *prefix.last().unwrap()) {
            prefix.pop();
            idx += 1;
            continue;
        } else {
            prefix.push(c1);
            idx += 1;
        }
    }
    prefix.into_iter().collect()
}

fn annihilate(c1: u8, c2: u8) -> bool {
    if c1 > c2 {
        c1 - c2 == 32
    } else {
        c2 - c1 == 32
    }
}

fn part2(input: &str) -> Result<()> {
    let result = part1_result(input);
    let mut min_len = u32::MAX;
    for m in 'a'..='z' {
        let mut clean = result.clone();
        clean = remove_impurities(clean, m as u8);
        clean = react(&mut clean);
        min_len = min(min_len, clean.len() as u32)
    }
    writeln!(std::io::stdout(), "min length={min_len}").ok();
    Ok(())
}

fn remove_impurities(clean: Vec<u8>, m: u8) -> Vec<u8> {
    clean
        .into_iter()
        .filter(|c| {
            let c = if c > &96u8 { c } else { &(*c + 32u8) };
            c != &m
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn annihilate_test1() {
        assert!(annihilate(b'A', b'a'));
    }
    #[test]
    fn annihilate_test2() {
        assert!(!annihilate(b'A', b'A'));
    }
    #[test]
    fn annihilate_test3() {
        assert!(!annihilate(b'A', b'b'));
    }
    #[test]
    fn part1_test1() {
        let input = "abcCBA";
        let result = part1_result(input);
        println!("result={result:?}");
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn part1_test2() {
        let input = "dabcCBA";
        let result = part1_result(input);
        println!("result={result:?}");
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn part1_test3() {
        let input = "dabAcCaCBAcCcaDA";
        let result = part1_result(input);
        assert_eq!(&result, "dabCBAcaDA".as_bytes());
    }
    #[test]
    fn part1_test4() {
        let input = "abcdeEDCfghIiHGkl";
        let result = part1_result(input);
        assert_eq!(&result, "abfkl".as_bytes());
    }
}
