use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;
use itertools::Itertools;


pub fn count_group(input: &str) -> usize {
    let person = input.split('\n').collect_vec();
    person.into_iter().join("").as_str().chars().unique().count()
}

pub fn sum_groups(input: &str) -> usize {
    let passport = input.split("\n\n");
    passport.map(count_group).sum()
}

pub fn count_group_two(input: &str) -> usize {
    let person = input.split('\n').collect_vec();
    let unique_ans = person.iter().join("").as_str().chars().unique().collect_vec();
    println!("All answers: {:?} : Individual answers {:?}", unique_ans, person);
    unique_ans.iter()
        .filter(|&&c|
            person.iter().all(|&person_ans|
                person_ans.chars().any(|p| p == c)))
        .count()
}

pub fn sum_groups_two(input: &str) -> usize {
    let passport = input.split("\n\n");
    passport.map(count_group_two).sum()
}

pub fn from_file<P>(name: P) -> usize where P: AsRef<Path> {
    let file = File::open(name);
    let mut buffer: String = String::new();
    match io::BufReader::new(file.unwrap()).read_to_string(&mut buffer) {
        Ok(_) => sum_groups(buffer.as_str()),
        _ => 0
    }
}

pub fn from_file_two<P>(name: P) -> usize where P: AsRef<Path> {
    let file = File::open(name);
    let mut buffer: String = String::new();
    match io::BufReader::new(file.unwrap()).read_to_string(&mut buffer) {
        Ok(_) => sum_groups_two(buffer.as_str()),
        _ => 0
    }
}

fn main() {
    println!("Sum of counts ONE: {}", from_file("./day-six/answers.dat"));
    println!("Sum of counts TWO: {}", from_file_two("./day-six/answers.dat"))
}

#[cfg(test)]
mod tests {
    use crate::{sum_groups, sum_groups_two};

    #[test]
    fn su_test() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
        assert_eq!(sum_groups(input), 11);
        assert_eq!(sum_groups_two(input), 6);
    }
}
