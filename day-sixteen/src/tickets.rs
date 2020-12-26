use std::ops::{Range, RangeInclusive};
use std::pin::Pin;
use std::str::FromStr;
use itertools::Itertools;
use debug_print::{debug_print};

struct RuleParseError;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule {
    name: String,
    allowed_ranges: (RangeInclusive<u16>, RangeInclusive<u16>),
}

impl Rule {
    pub fn validate_ticket(&self, ticket: &Vec<u16>) -> Result<(), Vec<u16>> {
        let mismatched = ticket.iter().filter(|val|
            !self.allowed_ranges.0.contains(val) && !self.allowed_ranges.1.contains(val)
        ).cloned().collect_vec();
        match mismatched[..] {
            [] => Ok(()),
            _ => Err(mismatched)
        }
    }
    pub fn validate_number(&self, val: u16) -> Result<(), u16> {
        match !self.allowed_ranges.0.contains(&val) && !self.allowed_ranges.1.contains(&val) {
            false => Ok(()),
            _ => Err(val)
        }
    }
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

impl Rules {
    pub fn validate_ticket(&self, ticket: &Vec<u16>) -> Result<(), Vec<u16>> {
        let mut unmatched = ticket.clone();
        for rule in self.0.iter() {
            match rule.validate_ticket(&unmatched) {
                Ok(()) => return Ok(()),
                Err(x) => { unmatched = x; }
            }
        }
        Err(unmatched)
    }
    pub fn validate_number(&self, num: u16) -> Result<(), u16> {
        if self.0.iter().any(|rule| rule.validate_number(num).is_err()) {
            Err(num)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TicketsData {
    rules: Rules,
    your_ticket: Vec<u16>,
    nearby_tickets: Vec<Vec<u16>>,
}

impl FromStr for TicketsData {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let your_ticket = str.find("\nyour ticket:").unwrap_or(0);
        let nearby_tickets = str.find("\nnearby tickets:").unwrap_or(0);
        Ok(TicketsData {
            rules: Rules::from_str(&str[0..your_ticket]).unwrap(),
            your_ticket: str[your_ticket..nearby_tickets]
                .split(':')
                .skip(1).next()
                .unwrap().trim()
                .split_terminator(',')
                .map(|s| s.parse::<u16>())
                .filter_map(Result::ok)
                .collect_vec(),
            nearby_tickets: str[nearby_tickets..]
                .split(":")
                .skip(1).next().unwrap_or("").trim().lines().map(|line|
                line.trim()
                    .split_terminator(',')
                    .map(|s| s.parse::<u16>())
                    .filter_map(Result::ok)
                    .collect_vec()
            ).collect_vec(),
        })
    }
}

impl TicketsData {
    pub fn scanning_error_rate(&self) -> u32 {
        self.nearby_tickets.iter()
            .map(|ticket| self.rules.validate_ticket(ticket))
            .filter_map(Result::err)
            .map(|ticket_errors| ticket_errors.iter().sum::<u16>() as u32)
            .sum()
    }
    pub fn valid_tickets(&self) -> Vec<Vec<u16>> {
        self.nearby_tickets.iter()
            .filter(|&ticket| self.rules.validate_ticket(ticket).is_ok())
            .cloned().collect_vec()
    }
    pub fn sort_rules(&mut self) -> &mut Self {
        let mut sorted_rules = self.rules.0.clone();
        let valid_tickets = self.valid_tickets();
        // Vector of rules that can be applied to field
        let mut field_rule_candidates = (0..self.your_ticket.len())
            .map(|_| (0..self.rules.0.len()).collect_vec())
            .collect_vec();
        for ticket in valid_tickets {
            ticket.iter().zip(field_rule_candidates.iter_mut()).for_each(|(&field, candidates)| {
                candidates.iter().position(|&idx| self.rules.0[idx].validate_number(field).is_err())
                    .map(|idx| candidates.remove(idx));
            });
        }
        loop {
            // Find rule which is only rule applicable to field
            let single_candidate = field_rule_candidates.iter().enumerate()
                .find(|(_, fp)| fp.len() == 1);
            let (i, poss) = match single_candidate {
                Some((i, v)) => (i, v[0]),
                None => break
            };
            sorted_rules[i] = self.rules.0[poss].clone();
            for fp in field_rule_candidates.iter_mut()
            {
                fp.iter().position(|&idx| idx == poss)
                    .map(|idx| fp.remove(idx));
            }
        }
        self.rules = Rules(sorted_rules);
        self
    }

    pub fn get_departure_multiple(&self) -> u64 {
        self.rules.0.iter().enumerate().filter(
            |(_, rule)| rule.name.contains("departure")
        )
            .map(|(index, rule)| {
                debug_print!("{}: {:?} : {}\n",index,rule, self.your_ticket[index]);
                self.your_ticket[index] as u64
            }).product()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Generator, GeneratorState};
    use std::pin::Pin;
    use crate::tickets::{Rule, string_to_range, string_to_range_tuple, Rules, TicketsData};
    use std::str::FromStr;
    use itertools::Itertools;

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
        println!("Parsed data: {:#?}", TicketsData::from_str(input));
        assert_eq!(TicketsData::from_str(input).unwrap().scanning_error_rate(), 71);
        assert_eq!(TicketsData::from_str(input).unwrap().valid_tickets(), vec![vec![7, 3, 47]]);
    }

    #[test]
    fn parsing_test() {
        assert_eq!(string_to_range("1-3"), 1..=3);
        assert_eq!(string_to_range("5-7"), 5..=7);
        assert_eq!(string_to_range_tuple("1-3 or 5-7"), (1..=3, 5..=7));
        assert_eq!(Rule::from_str("class: 1-3 or 5-7").unwrap(), Rule { name: "class".to_string(), allowed_ranges: (1..=3, 5..=7) });
        assert_eq!(Rule::from_str("class: 1-3 or 5-7").unwrap().validate_ticket(&vec![7, 3, 47]), Err(vec![47]));
        assert_eq!(Rule::from_str("row: 6-11 or 33-44").unwrap(), Rule { name: "row".to_string(), allowed_ranges: (6..=11, 33..=44) });
        assert_eq!(Rule::from_str("seat: 13-40 or 45-50").unwrap(), Rule { name: "seat".to_string(), allowed_ranges: (13..=40, 45..=50) });
    }

    #[test]
    fn rules_test() {
        let input_str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50"#;
        assert_eq!(Rules::from_str(input_str).unwrap().0.len(), 3);
        let rules = Rules::from_str(input_str).unwrap();
        assert_eq!(rules.validate_ticket(&vec![7, 3, 47]), Ok(()));
        assert_eq!(rules.validate_ticket(&vec![40, 4, 50]), Err(vec![4]));
        assert_eq!(rules.validate_ticket(&vec![55, 2, 20]), Err(vec![55]));
        assert_eq!(rules.validate_ticket(&vec![38, 6, 12]), Err(vec![12]));
    }

    #[test]
    fn part_two_example() {
        let input = r#"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9"#;
        let mut tickets_data = TicketsData::from_str(input).unwrap();
        assert_eq!(tickets_data.valid_tickets(), tickets_data.nearby_tickets);
        tickets_data.sort_rules();
        println!("Sorted rules: {:#?}", tickets_data.rules);
        assert_eq!(tickets_data.rules.0.iter().map(|rule| rule.name.as_str()).collect_vec(), vec!["row", "class", "seat"]);
        // assert_eq!(tickets_data.sort_rules().get_departure_multiple(),0)
    }
}