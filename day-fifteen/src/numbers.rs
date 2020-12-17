use std::collections::HashMap;
use itertools::__std_iter::FromIterator;
use debug_print::debug_print;

struct SpokenAt {
    prev: u64,
    last: u64
}
pub struct NumberGenerator{
    last_spoken: u64,
    index: u64,
    numbers: HashMap<u64, (u64,u64)>
}

impl From<&[u64]> for NumberGenerator {
    fn from(initial: &[u64]) -> Self {
        NumberGenerator{
            last_spoken: *initial.last().unwrap(),
            index: initial.len() as u64 +1,
            numbers: HashMap::from_iter(
                initial.iter().enumerate().map(|(i, &num)|
                    (num,((i+1) as u64, 0))
                )
            )
        }
    }
}
impl Iterator for NumberGenerator{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // let val = self.numbers.get(&self.last_spoken).cloned();
        match self.numbers.get_mut(&self.last_spoken){
            Some((previous,last )) if *last == 0=> {
                debug_print!("Turn {}: {} starting number spoken at {} then {}", self.index, self.last_spoken, previous, last);
                self.last_spoken = 0;
                self.update_last_spoken();
                debug_print!(" returning {}\n",self.last_spoken);
                self.index += 1;
                Some(self.last_spoken)
            },
            Some((previous,last ))=> {
                debug_print!("Turn {}: {} number spoken at {} then {}", self.index, self.last_spoken, previous, last);
                // self.numbers.insert(self.last_spoken,(last, self.index));
                self.last_spoken = *last-*previous;
                self.update_last_spoken();
                self.index += 1;
                debug_print!(" returning {}\n",self.last_spoken);
                Some(self.last_spoken)
            },
            _ => unimplemented!()
        }
    }
}

impl NumberGenerator {
    fn update_last_spoken(&mut self) {
        match self.numbers.get_mut(&self.last_spoken) {
            Some(val) if val.1 == 0 => *val = (val.0, self.index),
            Some(val) => *val = (val.1, self.index),
            None => { self.numbers.insert(self.last_spoken, (self.index, 0)); }
        }
    }
}
pub fn spoken_number(generator: &mut NumberGenerator, index: usize) ->u64{
    for x in 0..(index- generator.index as usize) { generator.next(); }
    generator.next().unwrap()
}

#[cfg(test)]
mod tests{
    use crate::numbers::{NumberGenerator, spoken_number};

    #[test]
    fn ctor_test() {
        let generator = NumberGenerator::from(vec![0,3,6].as_slice());
        // println!("{:#?}", generator.numbers);
        assert_eq!(generator.last_spoken, 6);
    }
    #[test]
    fn first_example_test() {
        let mut generator = NumberGenerator::from(vec![0,3,6].as_slice());
        println!("{:?}", generator.numbers);
        assert_eq!(generator.next().unwrap(),0);
        // println!("{:?}", generator.numbers);
        assert_eq!(generator.next().unwrap(),3);
        // println!("{:?}", generator.numbers);
        assert_eq!(generator.next().unwrap(),3);
        assert_eq!(generator.next().unwrap(),1);
        assert_eq!(generator.next().unwrap(),0);
        assert_eq!(generator.next().unwrap(),4);
        assert_eq!(generator.next().unwrap(),0);
    }
    #[test]
    fn get_2020th_number(){
        {
            let mut generator = NumberGenerator::from(vec![0, 3, 6].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 436);
        }
        {
            let mut generator = NumberGenerator::from(vec![1,3,2].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 1);
        }
        {
            let mut generator = NumberGenerator::from(vec![2,1,3].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 10);
        }
        {
            let mut generator = NumberGenerator::from(vec![1,2,3].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 27);
        }
        {
            let mut generator = NumberGenerator::from(vec![2,3,1].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 78);
        }
        {
            let mut generator = NumberGenerator::from(vec![3,2,1].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 438);
        }
        {
            let mut generator = NumberGenerator::from(vec![3,1,2].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 1836);
        }
    }

    #[test]
    fn get_2020th_number_from_puzzle_input() {
        {
            let mut generator = NumberGenerator::from(vec![16,11,15,0,1,7].as_slice());
            assert_eq!(spoken_number(&mut generator, 2020), 662);
        }
    }
    #[test]
    fn get_30000000th_number_from_puzzle_input() {
        {
            let mut generator = NumberGenerator::from(vec![16,11,15,0,1,7].as_slice());
            assert_eq!(spoken_number(&mut generator, 30000000), 662);
        }
    }
}


