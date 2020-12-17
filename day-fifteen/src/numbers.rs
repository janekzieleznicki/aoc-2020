use std::collections::HashMap;
use itertools::__std_iter::FromIterator;
use debug_print::debug_print;
struct SpokenAt {
    prev: u64,
    last: u64
}
struct NumberGenerator{
    last_spoken: u64,
    index: u64,
    numbers: HashMap<u64, (u64,u64)>
}

impl From<&[u64]> for NumberGenerator {
    fn from(initial: &[u64]) -> Self {
        NumberGenerator{
            last_spoken: *initial.last().unwrap(),
            index: initial.len() as u64,
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
        match self.numbers.get(&self.last_spoken).cloned(){
            Some((last,0 ))=> {
                debug_print!("{} starting number spoken at {} then {}\n", self.last_spoken, 0, last);
                self.index += 1;
                match self.numbers.get_mut(&0u64) {
                    Some(x) => {
                        x=(x.1, self.index);
                    }
                }
                self.numbers.insert(self.numbers.get_mut(0),(last,self.index));
                self.last_spoken = 0;
                Some(self.last_spoken)
            },
            Some((previous,last ))=> {
                debug_print!("{} starting number spoken at {} then {}\n", self.last_spoken, previous, last);
                self.index += 1;
                self.numbers.insert(0,(last, self.index));
                self.last_spoken = last-previous;
                Some(last-previous)
            },
            _ => unimplemented!()
        }
    }
}

// pub fn spoken_number(index: usize)->usize{
//
// }

#[cfg(test)]
mod tests{
    use crate::numbers::NumberGenerator;

    #[test]
    fn ctor_test() {
        let generator = NumberGenerator::from(vec![0,3,6].as_slice());
        println!("{:#?}", generator.numbers);
        assert_eq!(generator.last_spoken, 6);
    }
    #[test]
    fn first_example_test() {
        let mut generator = NumberGenerator::from(vec![0,3,6].as_slice());
        assert_eq!(generator.next().unwrap(),0);
        assert_eq!(generator.next().unwrap(),3);
        assert_eq!(generator.next().unwrap(),3);
        assert_eq!(generator.next().unwrap(),1);
    }
}

