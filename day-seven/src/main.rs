#![feature(str_split_once)]

use std::path::Path;
use std::str::FromStr;
use std::fs::File;
use itertools::Itertools;
use std::collections::{HashSet, HashMap};
use std::io;
use std::io::BufRead;

fn main() {
    let rules = BagRule::from_file("./day-seven/bags.dat");
    println!("Result: {:#?}", validate_bag(&rules, &Bag { color: "shiny gold".to_string() }));
    println!("Part two: {:#?}", inside_bag(&rules, &Bag { color: "shiny gold".to_string() }));

}

#[derive(Debug, Clone)]
pub struct BagParseError {}

#[derive(Hash, Eq, Clone, Debug, PartialEq)]
pub struct Bag {
    color: String
}

#[derive(Debug, PartialEq)]
struct AllowedBag {
    color: String,
    count: usize,
}

impl From<&AllowedBag> for Bag {
    fn from(allowed: &AllowedBag) -> Self {
        Self {
            color: allowed.color.clone()
        }
    }
}

impl AllowedBag {
    pub fn allowed(&self, bag: &Bag) -> Option<Bag> {
        match bag {
            _ if bag.color == self.color => Some(self.into()),
            _ => None
        }
    }
}

impl FromStr for AllowedBag {
    type Err = BagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().split_once(char::is_whitespace) {
            Some((count, color)) => Ok(AllowedBag {
                count: match count.parse::<usize>() {
                    Ok(c) => c,
                    Err(_) => 0
                },
                color: color.trim_start_matches(char::is_whitespace)
                    .trim_end_matches(" bag")
                    .trim_end_matches(" bags")
                    .to_string(),
            }),
            None => {
                eprintln!("AllowedBag unable to parse: {}", s);
                Err(Self::Err {})
            }
        }
    }
}

#[derive(Debug)]
pub struct BagRule {
    color: String,
    allowed: Vec<AllowedBag>,
}

impl BagRule {
    pub fn from_file<P>(name: P) -> Vec<BagRule> where P: AsRef<Path> {
        let file = File::open(name);
        io::BufReader::new(file.unwrap()).lines().filter_map(|res| res.ok()).map(|line| BagRule::from_str(line.as_str()).unwrap()).collect()
    }
    pub fn from_string(str: &str) -> Vec<BagRule> {
        str.lines().map(|line| BagRule::from_str(line).unwrap()).collect()
    }
    pub fn allowed(&self, bag: &Bag) -> Option<Bag> {
        match self.allowed.iter().any(|rule| rule.allowed(bag).is_some()) {
            true => Some(Bag { color: self.color.clone() }),
            false => None
        }
    }
    // pub fn insides(&self, bag: &Bag) -> HashMap<Bag,usize> {
    //     // match self.allowed.iter().any(|rule| rule.allowed(bag).is_some()) {
    //     //     true => Some(Bag{color: self.color.clone()}),
    //     //     false=> None
    //     // }
    // }
}

pub fn validate_bag(bag_rules: &[BagRule], bag: &Bag) -> usize {
    let mut bags_to_check = vec![bag.clone()];
    let mut can_contain: HashSet<Bag> = HashSet::new();
    loop {
        bags_to_check = bags_to_check.iter()
            .map(|b|
                bag_rules.iter()
                    .map(|f| f.allowed(b))
                    .filter_map(|res| res)
                    .collect::<Vec<Bag>>()
            ).flatten()
            .unique()
            .filter(|fb| !can_contain.contains(fb))
            .collect();
        println!("Checking: {:#?}", bags_to_check);
        println!("Matched {:#?}\n\n", can_contain);
        match bags_to_check[..] {
            [] => break,
            _ => {
                bags_to_check.iter().for_each(|bag| { can_contain.insert(bag.clone()); });
                continue;
            }
        }
    }
    can_contain.len()
}

pub fn inside_bag(bag_rules: &[BagRule], bag: &Bag) -> usize {
    let mut iter: usize = 0;
    let mut bags: HashMap<Bag, usize> = HashMap::new();
    bags.insert(bag.clone(), 1);
    loop {
        let mut new_bags: HashMap<Bag, usize> = HashMap::new();
        bags.iter()
            .map(|(bag, count)| bag_rules.iter()
                .filter(|&rule| rule.color == bag.color)
                .map(|rule| {
                    // println!("Updating {:?}, with {}", bag, count);
                    // match rule.allowed[..] {
                    //     [] => vec![(Bag { color: rule.color.clone() }, 0 * count)],
                    //     _ => rule.allowed.iter().map(|all| (Bag { color: all.color.clone() }, all.count * count)).collect::<Vec<(Bag, usize)>>()
                    // }
                    rule.allowed.iter().map(|all| (Bag { color: all.color.clone() }, match all.count {
                        0 => *count,
                        _ => all.count * count
                    })).collect::<Vec<(Bag, usize)>>()
                }).flatten().collect::<Vec<(Bag, usize)>>()
            ).flatten().collect::<Vec<(Bag, usize)>>().iter()
            .for_each(|(bag, count)| {
                // println!("Updating {:?}, with {}", bag, count);
                let curr = new_bags.entry(bag.clone()).or_insert(0);
                *curr += count;
            });

        println!("---------------\nBags: {:#?}\n\nNew Bags:{:#?}", bags, new_bags);
        // if iter >= 1 {
        //     break;
        // }
        // iter += 1;
        match new_bags {
            _ if new_bags.is_empty() => break,
            _ => {
                // new_bags.into_iter().for_each(|(key, val)| {
                //     let curr = bags.entry(key).or_default();
                //     *curr += val;
                // });
                bags = new_bags;
                iter += bags.values().sum::<usize>();
                continue;
            }
        }
    }
    // bags.values().sum()
    iter
}

impl FromStr for BagRule {
    type Err = BagParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("bags contain") {
            Some((color, allowed)) => Ok(BagRule {
                color: color.trim().to_string(),
                allowed: allowed.split(|c| char::is_ascii_punctuation(&c))
                    .map(str::trim).filter(|str| !str.is_empty())
                    .map(AllowedBag::from_str)
                    .map(|res| res.unwrap())
                    .filter(|res| !res.color.eq("other"))
                    .collect(),
            }),

            None => Err(Self::Err {})
        }
    }
}

#[cfg(test)]
mod bags {
    use crate::{BagRule, AllowedBag, validate_bag, Bag, inside_bag};
    use std::str::FromStr;

    #[test]
    fn allowed_bag_test() {
        {
            assert_eq!(AllowedBag::from_str("1 bright white bag").unwrap(), AllowedBag { color: "bright white".to_string(), count: 1 });
            assert_eq!(AllowedBag::from_str(" 2 muted yellow bags").unwrap(), AllowedBag { color: "muted yellow".to_string(), count: 2 });
        }
    }

    #[test]
    fn single_rule() {
        let rule = BagRule::from_str("bright white bags contain 1 shiny gold bag.").unwrap();
        println!("Rule: {:#?}", rule);
        assert_eq!(rule.allowed(&Bag { color: "shiny gold".to_string() }), Some(Bag { color: "bright white".to_string() }));
    }

    #[test]
    fn ex_test() {
        let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
        let rules = BagRule::from_string(input);
        assert_eq!(rules.len(), 9);
        // println!("{:#?}", rules);
        assert_eq!(validate_bag(&rules, &Bag { color: "shiny gold".to_string() }), 4)
    }

    #[test]
    fn inside_bag_test() {
        let input = r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."#;
        let rules = BagRule::from_string(input);
        println!("{:#?}", rules);
        assert_eq!(inside_bag(&rules, &Bag { color: "shiny gold".to_string() }), 126)
    }
}