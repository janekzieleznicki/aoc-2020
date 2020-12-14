use itertools::{sorted, Itertools};
use debug_print::{debug_print};
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;

pub fn from_file<P>(name: P) -> Vec<u32> where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    io::BufReader::new(file).lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn main() {
    let u32_input = from_file("./day-ten/adapters.dat");
    let mut bag = AdapterBag::from(&*u32_input.into_iter().map_into::<Adapter>().collect_vec());
    println!("Possible adapter chain count: {}", bag.all_chains_from_wall());
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Adapter {
    rating: u32
}

impl From<u32> for Adapter {
    fn from(u: u32) -> Self {
        Adapter { rating: u }
    }
}

impl Adapter {
    pub fn compatible(&self, right: &Adapter) -> bool {
        self.rating < right.rating
            &&
            (right.rating as i64 - self.rating as i64) <= 3
    }
}

struct AdapterBag {
    adapters: Vec<Adapter>,
    device: Adapter,
    set_count: usize,
}

impl AdapterBag {
    fn single_chain(&self, used: &[Adapter], possible: &[Adapter]) -> Option<Vec<Vec<Adapter>>> {
        match possible.len() {
            0 if used.last().unwrap().compatible(&self.device) => {
                let mut res = used.to_vec();
                res.push(self.device);
                return Some(vec![res]);
            }
            _ => {}
        }
        let possibles = possible.iter()
            .take(1)
            .take_while(|&candidate| used.last().unwrap().compatible(candidate))
            .map(|valid_adapter| {
                let mut new_sued = used.to_vec();
                new_sued.push(valid_adapter.clone());
                let mut new_possible = possible.to_vec();
                new_possible.retain(|x| *x != *valid_adapter);
                self.single_chain(&*new_sued, &*new_possible)
            }).while_some().flatten().collect_vec();
        match possibles.is_empty() {
            true => None,
            false => Some(possibles)
        }
    }
    pub fn chain_from_wall(&self) -> Option<Vec<Vec<Adapter>>> {
        self.single_chain(&[Adapter { rating: 0 }], &self.adapters)
    }
    pub fn all_chain_from_wall_helper(rest: &[u128], cache: &mut HashMap<u128, u128>) -> u128 {
        debug_print!("Values {:?}\nCache: {:?}\n",rest,cache);
        match cache.get(rest.first().unwrap()) {
            Some(x) => return *x,
            None => {
                match rest.len() {
                    1 => {
                        cache.insert(*rest.first().unwrap(), 1);
                        return 1;
                    }
                    _ => {
                        let count = rest.iter().dropping(1).enumerate()
                            .take_while(|(i, &next)| (next - rest.first().unwrap()) <= 3)
                            .map(|(i, next)|
                                AdapterBag::all_chain_from_wall_helper(&rest[(i + 1)..], cache)
                            ).sum();
                        cache.insert(*rest.first().unwrap(), count);
                        return count;
                    }
                }
            }
        }
    }
    pub fn all_chains_from_wall(&mut self) -> u128 {
        let mut adapters: Vec<u128> = Vec::new();
        adapters.push(0);
        adapters.extend(self.adapters.iter().map(|adapter| adapter.rating as u128));
        adapters.push(self.device.rating as u128);
        AdapterBag::all_chain_from_wall_helper(&*adapters, &mut HashMap::new())
    }
}

fn number_of_diferences(chain: &[Adapter]) -> (usize, usize) {
    let differences = chain.windows(2).map(|w| w[1].rating - w[0].rating).collect_vec();
    (differences.iter().filter(|&x| *x == 1).count()
     , differences.iter().filter(|&x| *x == 3).count())
}

impl From<&[Adapter]> for AdapterBag {
    fn from(adapters: &[Adapter]) -> Self {
        let dev = adapters.iter().max().unwrap();
        AdapterBag {
            adapters: sorted(adapters).copied().collect_vec(),
            device: Adapter { rating: dev.rating + 3 },
            set_count: 0,
        }
    }
}


#[cfg(test)]
mod joltage_tests {
    use crate::{AdapterBag, Adapter, number_of_diferences};
    use itertools::{Itertools, assert_equal};
    use debug_print::{debug_print};

    #[test]
    fn adapter_test() {
        assert!(!Adapter { rating: 0 }.compatible(&Adapter { rating: 11 }));
        assert!(!Adapter { rating: 11 }.compatible(&Adapter { rating: 0 }));
        assert!(!Adapter { rating: 11 }.compatible(&Adapter { rating: 9 }));
    }

    #[test]
    fn simple() {
        let input: Vec<u32> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let mut bag = AdapterBag::from(&*input.iter().copied().map_into::<Adapter>().collect_vec());
        assert_eq!(bag.device.rating, 22);
        assert_eq!(bag.adapters.last().unwrap(), &Adapter { rating: 19 });
        assert_eq!(bag.adapters.first().unwrap(), &Adapter { rating: 1 });
        if let Some(chains) = bag.chain_from_wall() {
            chains.iter().for_each(|chain| { debug_print!("{:?}\n", chain) });
            assert_eq!(number_of_diferences(&chains[0]), (7, 5));
        }
        assert_eq!(bag.all_chains_from_wall(), 8);
    }

    #[test]
    fn larger() {
        let input: Vec<u32> = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
        let mut bag = AdapterBag::from(&*input.iter().copied().map_into::<Adapter>().collect_vec());
        assert_eq!(bag.device.rating, 52);
        assert_eq!(bag.adapters.last().unwrap(), &Adapter { rating: 49 });
        assert_eq!(bag.adapters.first().unwrap(), &Adapter { rating: 1 });
        if let Some(chains) = bag.chain_from_wall() {
            chains.iter().for_each(|chain| { debug_print!("{:?}\n", chain) });
            assert_eq!(number_of_diferences(&chains[0]), (22, 10));
        }
        assert_eq!(bag.all_chains_from_wall(), 19208);
    }
}