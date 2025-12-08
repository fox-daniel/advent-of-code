use std::fs;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input/input.txt")?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = part1_result(input);
    writeln!(std::io::stdout(), "length: {}", result.len()).ok();
    Ok(())
}

fn part1_result(s: &str) -> String {
    let mut prefix: Vec<char> = vec![];
    let mut protein: Vec<char> = s.chars().collect();
    if protein.last().unwrap() == &'\n' {
        protein.pop();
    }
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
    prefix.iter().collect()
}

fn annihilate(c1: char, c2: char) -> bool {
    ((c1.is_lowercase() & c2.is_uppercase()) | (c2.is_lowercase() & c1.is_uppercase()))
        & (c1.to_lowercase().next() == c2.to_lowercase().next())
}

fn part2(_input: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn annihilate_test1() {
        assert!(annihilate('A', 'a'));
    }
    #[test]
    fn annihilate_test2() {
        assert!(!annihilate('A', 'A'));
    }
    #[test]
    fn annihilate_test3() {
        assert!(!annihilate('A', 'b'));
    }
    #[test]
    fn part1_test1() {
        let input = "abcCBA";
        let result = part1_result(input);
        println!("result={result}");
        assert_eq!(result.len(), 0);
    }
    #[test]
    fn part1_test2() {
        let input = "dabcCBA";
        let result = part1_result(input);
        println!("result={result}");
        assert_eq!(result.len(), 1);
    }
    #[test]
    fn part1_test3() {
        let input = "dabAcCaCBAcCcaDA";
        let result = part1_result(input);
        println!("result={result}");
        assert_eq!(&result, "dabCBAcaDA");
    }
    #[test]
    fn part1_test4() {
        let input = "abcdeEDCfghIiHGkl";
        let result = part1_result(input);
        println!("result={result}");
        assert_eq!(&result, "abfkl");
    }
}
