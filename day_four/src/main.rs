use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

pub trait EntryValidator {
    fn validate(&self, key: &str, val: &str) -> bool;
}

pub struct SimpleEntryValidator {}

impl SimpleEntryValidator {
    const VALID_KEYS: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
}

impl EntryValidator for SimpleEntryValidator {
    fn validate(&self, key: &str, value: &str) -> bool {
        match key {
            _ if SimpleEntryValidator::VALID_KEYS.contains(&key) => match value {
                "" => false,
                _ => true
            }
            _ => false
        }
    }
}

pub struct RequiringValidator {}

impl RequiringValidator {
    fn byr(&self, val: &str) -> bool {
        match val.len() {
            4 => {
                let x = val.parse::<usize>();
                match x {
                    Ok(x) => match x {
                        1920..=2020 => true,
                        _ => false
                    }
                    _ => false
                }
            }
            _ => false
        }
    }
    fn iyr(&self, val: &str) -> bool {
        match val.len() {
            4 => {
                let x = val.parse::<usize>();
                match x {
                    Ok(x) => match x {
                        2010..=2020 => true,
                        _ => false
                    }
                    _ => false
                }
            }
            _ => false
        }
    }
    fn eyr(&self, val: &str) -> bool {
        match val.len() {
            4 => {
                let x = val.parse::<usize>();
                match x {
                    Ok(x) => match x {
                        2020..=2030 => true,
                        _ => false
                    }
                    _ => false
                }
            }
            _ => false
        }
    }
    fn hgt(&self, val: &str) -> bool {
        fn hgt_cm(val: &str) -> bool {
            match val.matches(char::is_numeric).collect::<String>().parse::<usize>() {
                Ok(hight) => match hight {
                    150..=193 => true,
                    _ => false
                }
                _ => false
            }
        }
        fn hgt_in(val: &str) -> bool {
            match val.matches(char::is_numeric).collect::<String>().parse::<usize>() {
                Ok(hight) => match hight {
                    59..=76 => true,
                    _ => false
                }
                _ => false
            }
        }
        match val {
            _ if val.ends_with("cm") => hgt_cm(val),
            _ if val.ends_with("in") => hgt_in(val),
            _ => false
        }
    }
    fn hcl(&self, val: &str) -> bool {
        if val.starts_with('#') && val.len() == 7 {
            val.strip_prefix('#').unwrap().matches(char::is_alphanumeric).count() == 6
        } else {
            false
        }
    }
    fn ecl(&self, val: &str) -> bool {
        match val {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    }
    fn pid(&self, val: &str) -> bool {
        val.matches(char::is_numeric).count()==9
    }
}

impl EntryValidator for RequiringValidator {
    fn validate(&self, key: &str, value: &str) -> bool {
        match key {
            "byr" => self.byr(value),
            "iyr" => self.iyr(value),
            "eyr" => self.eyr(value),
            "hgt" => self.hgt(value),
            "hcl" => self.hcl(value),
            "ecl" => self.ecl(value),
            "pid" => self.pid(value),
            _ => false
        }
    }
}

struct PassportValidator {
    entry_validator: Box<dyn EntryValidator>
}

impl PassportValidator {
    pub fn key_val<'a>(&self, str: &'a str) -> (&'a str, &'a str) {
        let vec = str.split(":").collect::<Vec<&str>>();
        (vec[0], vec[1])
    }
    pub fn validate_passport(&self, input: &str) -> bool {
        let mut expected = vec![
            "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"
        ];
        expected.sort();
        let mut entries = input.split_whitespace().map(|entry|
            self.key_val(entry)).filter(|(key, val)| self.entry_validator.validate(key, val))
            .map(|(key, _)| key)
            .collect::<Vec<&str>>();
        entries.sort();
        println!("Validating {} \nEntries:\t{}\nExpectesd:\t{}", expected.eq(&entries),
                 entries.clone().join("|"),
                 expected.clone().join("|"));
        expected.eq(&entries)
        // expected.eq(&entries)
    }

    pub fn validate(&self, input: &str) -> usize {
        let passport = input.split("\n\n");
        passport.filter(|passport|
            self.validate_passport(passport)).count()
    }
    pub fn from_file<P>(&self, name: P) -> usize where P: AsRef<Path> {
        let file = File::open(name);
        let mut buffer: String = String::new();
        match io::BufReader::new(file.unwrap()).read_to_string(&mut buffer){
            Ok(_) => self.validate(buffer.as_str()),
            Err(_) => 0
        }

    }
}


fn main() {
    let res_1 = PassportValidator { entry_validator: Box::new(SimpleEntryValidator {}) }.from_file("./day_four/passports.dat");
    let res_2 = PassportValidator { entry_validator: Box::new(RequiringValidator {}) }.from_file("./day_four/passports.dat");
    println!("Valid passports count ONE: {}", res_1);
    println!("Valid passports count TWO: {}", res_2);
}

#[cfg(test)]
mod tests {
    use crate::{SimpleEntryValidator, PassportValidator, RequiringValidator};

    const RAW_INPUT: &str =
        r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn test() {
        assert_eq!(PassportValidator { entry_validator: Box::new(SimpleEntryValidator {}) }.validate(RAW_INPUT), 2);
    }

    #[test]
    fn part_two_valid() {
        const INPUT: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"#;
        assert_eq!(PassportValidator { entry_validator: Box::new(RequiringValidator {}) }.validate(INPUT), 4);
    }

    #[test]
    fn part_two_invalid() {
        const INPUT: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
        assert_eq!(PassportValidator { entry_validator: Box::new(RequiringValidator {}) }.validate(INPUT), 0);
    }
}