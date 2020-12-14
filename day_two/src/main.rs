use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct PasswordPolicy {
    min: usize,
    max: usize,
    leter: char,
}
impl PasswordPolicy {
    pub fn sled_rentel_place_check(&self, password: &str) -> bool {
        let range = self.min ..self.max+1 ;
        range.contains(&password.matches(self.leter).count() )
    }
    pub fn toboggan_chek(&self, password: &str) -> bool {
        let (first,second) = (password.as_bytes()[self.min-1] as char,password.as_bytes()[self.max-1] as char);
        if first == second{
            return false
        }
        else if first == self.leter || second==self.leter {
            return true
        }
        false
    }
}
impl FromStr for PasswordPolicy {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let range: Vec<&str> = iter.next().unwrap().split("-").collect();
        let letter = iter.next().unwrap().parse::<char>().unwrap();
        Ok(PasswordPolicy {
            min: range[0].parse::<usize>()?,
            max: range[1].parse::<usize>()?,
            leter: letter,
        })
    }
}

pub fn read_lines<P>(name: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>
{
    let file = File::open(name)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_single(str: &str) -> (PasswordPolicy, String) {
    let iter: Vec<&str> = str.split(": ").collect();
    (PasswordPolicy::from_str(iter[0]).unwrap(), String::from(iter[1]))
}

pub fn read_data<P>(name: P) -> Vec<(PasswordPolicy, String)> where P: AsRef<Path> {
    read_lines(name).unwrap().map(|line| read_single(&*line.unwrap())).collect()
}

fn main() {
    let polcied_pass = read_data("./day_two/input.dat");
    // assert_eq!(polcied_pass.len(),1000);
    polcied_pass.iter().for_each(|(policy,pass)|println!("{:?}\t'{}'\t=>\t{}",policy,pass,policy.sled_rentel_place_check(pass)));
    println!("Correct password count: {}",polcied_pass.iter().filter(|(policy,pass)|policy.sled_rentel_place_check(pass)).count());
    println!("Official Toboggan Corporate Policy password count: {}",polcied_pass.iter().filter(|(policy,pass)|policy.toboggan_chek(pass)).count());
}

#[cfg(test)]
mod tests {
    use crate::{PasswordPolicy, read_single};
    use std::str::FromStr;

    #[test]
    fn load() {
        let input = vec![
            ("7-8 x", PasswordPolicy { min: 7, max: 8, leter: 'x' }),
            ("9-11 k", PasswordPolicy { min: 9, max: 11, leter: 'k' }),
            ("8-12 g", PasswordPolicy { min: 8, max: 12, leter: 'g' }),
            ("6-9 v", PasswordPolicy { min: 6, max: 9, leter: 'v' }),
        ];
        for (str, policy) in input.iter() {
            assert_eq!(PasswordPolicy::from_str(str).unwrap(), *policy)
        }
    }

    #[test]
    fn load_single() {
        {
            let input = "7-8 x";
            assert_eq!(PasswordPolicy::from_str(input).unwrap(), PasswordPolicy { min: 7, max: 8, leter: 'x' })
        }
        {
            let input = "9-11 k";
            assert_eq!(PasswordPolicy::from_str(input).unwrap(), PasswordPolicy { min: 9, max: 11, leter: 'k' })
        }
    }

    #[test]
    fn read_single_test() {
        let input = vec![
            "4-12 h: mcwvwwphwwbc",
            "6-11 g: gqgggvggggh",
            "9-15 x: xxxxxxxxxxxxxxsx",
            "16-18 t: rmqqtbtvttsdtjvbttl",
            "9-20 f: cllnvlfkfrwzpqxwqgnn",
            "9-18 v: vvvvvvvvzvvvvvvzvxvv",
        ];
        assert_eq!(read_single(input[0]), (PasswordPolicy { min: 4, max: 12, leter: 'h' }, String::from("mcwvwwphwwbc")))
    }
    #[test]
    fn verify_password() {
        let input = vec![
            "4-12 h: mcwvwwphhhh",
            "6-9 g: gqgggvggggh",
        ];
        for str in input.iter()
        {
            let (policy,pass) = read_single(str);
            assert!(policy.sled_rentel_place_check(&*String::from(pass)));
        }
    }
    #[test]
    fn negative_verify_password() {
        let input = vec![
            "4-12 h: mcwvwwphhh",
            "6-9 g: gqgggvggggggh",
        ];
        for str in input.iter()
        {
            let (policy,pass) = read_single(str);
            assert!(!policy.sled_rentel_place_check(&*String::from(pass)));
        }
    }
}
