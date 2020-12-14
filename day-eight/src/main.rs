#![feature(str_split_once)]
#![feature(iter_advance_by)]
#![feature(exclusive_range_pattern)]

use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instruction {
    Acc { increment: i32 },
    Jmp { increment: i32 },
    Nop { increment: i32 },
}

impl Instruction {
    pub fn from_line<P>(line: P) -> Instruction where P: AsRef<str> {
        match line.as_ref().split_once(char::is_whitespace) {
            Some(("acc", x)) => Instruction::Acc { increment: x.parse::<i32>().unwrap() },
            Some(("jmp", x)) => Instruction::Jmp { increment: x.parse::<i32>().unwrap() },
            Some(("nop", x)) => Instruction::Nop { increment: x.parse::<i32>().unwrap() },
            _ => Instruction::Nop { increment: 0 },
        }
    }
}

pub fn from_lines(lines: &str) -> Vec<(Instruction, usize)> {
    lines.lines().map(Instruction::from_line).map(|instr| (instr, 0)).collect()
}

#[derive(Clone)]
pub struct Code {
    accumulator: i32,
    instructions: Vec<(Instruction, usize)>,
}

impl Code {
    pub fn from_file<P>(name: P) -> Code where P: AsRef<Path> {
        let file = File::open(name).unwrap();
        Code {
            accumulator: 0,
            instructions: io::BufReader::new(file).lines()
                .map(|l| l.unwrap())
                .map(Instruction::from_line)
                .map(|instr| (instr, 0))
                .collect(),
        }
    }
    pub fn from_str<P>(lines: P) -> Code where P: AsRef<str> {
        Code {
            accumulator: 0,
            instructions: lines.as_ref().lines()
                .map(Instruction::from_line)
                .map(|instr| (instr, 0))
                .collect(),
        }
    }

    pub fn execute(&mut self) -> i32 {
        self.accumulator = 0;
        let mut index: usize = 0;
        loop {
            let ins = &mut self.instructions[index];
            match &ins {
                (Instruction::Jmp { increment: inc @ 0..=i32::MAX },  0) => {
                    index += inc.abs() as usize;
                }
                (Instruction::Jmp { increment: inc @ i32::MIN..0 },  0) => {
                    index -= inc.abs() as usize;
                }
                (Instruction::Acc { increment: inc },  0) => {
                    index += 1;
                    self.accumulator += *inc;
                }
                (Instruction::Nop { increment: _ },  0) => {
                    index += 1;
                }
                (inst, cnt) => {
                    println!("Breaking at: {:?} used {} times", inst, cnt);
                    break;
                }
            };

            let (inst, cnt) = &ins;
            println!("{:?} used {} times", inst, cnt);
            ins.1 += 1;
        }
        self.accumulator
    }
    pub fn execute_correctly(&mut self) -> Result<usize, usize> {
        self.accumulator = 0;
        let mut index: usize = 0;
        loop {
            if index == self.instructions.len() {
                return Ok(index);
            }
            let ins = &mut self.instructions[index];
            match &ins {
                (Instruction::Jmp { increment: inc @ 0..=i32::MAX },  0) => {
                    index += inc.abs() as usize;
                }
                (Instruction::Jmp { increment: inc @ i32::MIN..0 },   0) => {
                    index -= inc.abs() as usize;
                }
                (Instruction::Acc { increment: inc },   0) => {
                    index += 1;
                    self.accumulator += *inc;
                }
                (Instruction::Nop { increment: _ },  0) => {
                    index += 1;
                }
                (inst, cnt) => {
                    println!("Breaking at {}: {:?} used {} times", index, inst, cnt);
                    return Err(index);
                }
            };

            let (inst, cnt) = &ins;
            println!("{:?} used {} times", inst, cnt);
            ins.1 += 1;
        }
    }
}

fn try_until_correct(code: &mut Code) -> i32 {
    let mut index: usize = 0;
    {
        let mut code = code.clone().to_owned();
        match code.execute_correctly() {
            Ok(_) => return code.accumulator,
            Err(_) => {}
        }
    }


    loop{
        let mut changed_code = code.clone().to_owned();
        //Change instruction loop
        loop {
            index += 1;
            match changed_code.instructions[index] {
                (Instruction::Jmp { increment: inc }, count) => {
                    println!("\nAttempt {}, modifying {:?}", index, changed_code.instructions[index]);
                    changed_code.instructions[index] = (Instruction::Nop { increment: inc }, count);
                    break
                }
                (Instruction::Nop { increment: inc }, count) => {
                    println!("\nAttempt {}, modifying {:?}", index, changed_code.instructions[index]);
                    changed_code.instructions[index] = (Instruction::Nop { increment: inc }, count);
                    break
                }
                (_,_) => {}
            }
        }
        match changed_code.execute_correctly() {
            Ok(_) => return changed_code.accumulator,
            Err(_) => {}
        }
    }

}


fn main() {
    let mut code = Code::from_file("./day-eight/asm.dat");
    println!("Result {:#?}", code.execute());
    println!("part two {:#?}",try_until_correct(&mut Code::from_file("./day-eight/asm.dat")));
}


#[cfg(test)]
mod assembler_tests {
    use crate::{Instruction, Code, try_until_correct};

    #[test]
    fn parser() {
        assert_eq!(Instruction::from_line("nop +0"), Instruction::Nop { increment: 0 });

        assert_eq!(Instruction::from_line("acc +1"), Instruction::Acc { increment: 1 });
        assert_eq!(Instruction::from_line("acc -99"), Instruction::Acc { increment: -99 });

        assert_eq!(Instruction::from_line("jmp +4"), Instruction::Jmp { increment: 4 });
        assert_eq!(Instruction::from_line("jmp -3"), Instruction::Jmp { increment: -3 });
    }

    #[test]
    fn from_example() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let mut code = Code::from_str(input);
        assert_eq!(code.execute(), 5);
        // code.execute();
    }

    #[test]
    fn incorrectly_executed() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;
        let mut code = Code::from_str(input);
        match code.execute_correctly() {
            Ok(x) => panic!("Finished at {} with result {}", x, code.accumulator),
            Err(x) => println!("Failed at {} with result {}", x, code.accumulator)
        }
        assert_eq!(try_until_correct(&mut Code::from_str(input)), 8);
    }

    #[test]
    fn correctly_executed() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6"#;
        let mut code = Code::from_str(input);
        match code.execute_correctly() {
            Ok(x) => println!("Finished at {} with result {}", x, code.accumulator),
            Err(x) => panic!("Failed at {} with result {}", x, code.accumulator)
        }
        assert_eq!(try_until_correct(&mut Code::from_str(input)), code.accumulator);
    }
}