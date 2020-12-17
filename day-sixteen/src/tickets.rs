use std::ops::{Generator, GeneratorState, Range, RangeInclusive};
use std::pin::Pin;
use std::str::FromStr;
use itertools::Itertools;

struct RuleParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    name: String,
    allowed_ranges: (RangeInclusive<u16>, RangeInclusive<u16>),
}

pub fn string_to_range(s: &str) -> RangeInclusive<u16> {
    match s.trim().split_once('-') {
        Some((left, right)) => {
            RangeInclusive::new(left.parse::<u16>().unwrap(), right.parse::<u16>().unwrap())
        }
        None => unimplemented!()
    }
}

pub fn string_to_range_tuple(s: &str) -> (RangeInclusive<u16>, RangeInclusive<u16>) {
    match s.split_once(" or ") {
        Some((left, right)) => {
            (string_to_range(left), string_to_range(right))
        }
        None => unimplemented!()
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.split(':').next().unwrap_or("");
        match s.split_once(':') {
            Some((name, ranges)) => {
                Ok(Rule {
                    name: name.to_string(),
                    allowed_ranges: string_to_range_tuple(ranges),
                })
            }
            None => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Rules(Vec<Rule>);

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rules(s.lines().map(|str| Rule::from_str(str).unwrap()).collect_vec()))
    }
}


#[cfg(test)]
mod tests {
    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;
    use crate::tickets::{Rule, string_to_range, string_to_range_tuple, Rules};
    use std::str::FromStr;

    #[test]
    fn test_from_example() {
        let input = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;
        let mut generator = move |str: &str| {
            let your_ticket = str.find("\nyour ticket:").unwrap_or(0);
            yield &str[0..your_ticket];
            let nearby_tickets = str.find("\nnearby tickets:").unwrap_or(0);
            yield &str[your_ticket..nearby_tickets].split(':').skip(1).next().unwrap_or("");
            return str[nearby_tickets..].split(':').skip(1).next().unwrap_or("");
        };
        match Pin::new(&mut generator).resume(input) {
            GeneratorState::Yielded(x) => { println!("Rules: {} \n--------------", x); }
            _ => panic!("unexpected value from resume"),
        }
        match Pin::new(&mut generator).resume(input) {
            GeneratorState::Yielded(x) => { println!("Your: {} \n--------------", x) }
            _ => panic!("unexpected value from resume"),
        }
        match Pin::new(&mut generator).resume(input) {
            GeneratorState::Complete(x) => { println!("Nearby: {} \n--------------", x) }
            _ => panic!("unexpected value from resume"),
        }
    }

    #[test]
    fn parsing_test() {
        assert_eq!(string_to_range("1-3"), 1..=3);
        assert_eq!(string_to_range("5-7"), 5..=7);
        assert_eq!(string_to_range_tuple("1-3 or 5-7"), (1..=3, 5..=7));
        assert_eq!(Rule::from_str("class: 1-3 or 5-7").unwrap(), Rule { name: "class".to_string(), allowed_ranges: (1..=3, 5..=7) });
        assert_eq!(Rule::from_str("row: 6-11 or 33-44").unwrap(), Rule { name: "row".to_string(), allowed_ranges: (6..=11, 33..=44) });
        assert_eq!(Rule::from_str("seat: 13-40 or 45-50").unwrap(), Rule { name: "seat".to_string(), allowed_ranges: (13..=40, 45..=50) });
    }
}