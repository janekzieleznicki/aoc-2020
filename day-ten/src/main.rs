use itertools::{sorted, Itertools};
use debug_print::{debug_print};
use reduce::Reduce;
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
    // if let Some(chains) = bag.chain_from_wall() {
    //     chains.iter().for_each(|chain| { debug_print!("{:?}\n", chain) });
    //     let (one_diff_count, three_diff_coiunt) = number_of_diferences(&chains[0]);
    //     println!("Differences count: one:{} | three: {}, Product: {}", one_diff_count, three_diff_coiunt, one_diff_count * three_diff_coiunt);
    // }
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

// impl From<&{integr}>
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
    fn all_chains(&mut self, used: &[Adapter], possible: &[Adapter]) {
        if used.last().unwrap().compatible(&self.device) {
            self.set_count += 1;
        } else {
            debug_print!("Used: {:?} | Possible: {:?}\n\n",used,possible);
            // let mut res = Vec::new();
            for x in 0..possible.len() {
                if used.last().unwrap().compatible(&possible[x]) {
                    let mut new_sued = used.to_vec();
                    new_sued.push(possible[x].clone());
                    debug_print!("Sending Used: {:?}, Possible: {:?}",&new_sued,&possible[x + 1..]);
                    self.all_chains(&*new_sued, &possible[x + 1..]);
                } else if possible[x].rating - used.last().unwrap().rating > 3 {
                    debug_print!("Exiting iter due to difference between {} and {}\n",possible[x].rating,used.last().unwrap().rating);
                    break;
                } else {
                    debug_print!("Sending Used: {:?}, Possible: {:?}",&used,&possible[x + 1..]);
                    self.all_chains(&*used, &possible[x + 1..]);
                }
            }
        }
    }

    pub fn chain_from_wall(&self) -> Option<Vec<Vec<Adapter>>> {
        self.single_chain(&[Adapter { rating: 0 }], &self.adapters)
    }
    pub fn all_chain_from_wall_helper(rest: &[u128], cache: &mut HashMap<u128,u128>) -> u128 {
        debug_print!("Values {:?}\nCache: {:?}\n",rest,cache);
        match cache.get(rest.first().unwrap()){
            Some(x) => return *x,
            None => {
                if rest.len() == 1{
                    cache.insert(*rest.first().unwrap(),1);
                    return 1
                } else {
                    let mut count = 0;
                    let mut niter = rest.iter();
                    niter.next();
                    for (i, next) in niter.enumerate() {
                        if next-rest.first().unwrap() <= 3 {
                            let cn = AdapterBag::all_chain_from_wall_helper(&rest[(i+1)..],cache);
                            count+=cn;
                        } else {
                            break;
                        }
                    }
                    cache.insert(*rest.first().unwrap(),count);
                    return count
                }
            }
        }
    }
    pub fn all_chains_from_wall(&mut self) -> u128 {
        // let plug = self.adapters.clone();
        // self.all_chains(&[Adapter { rating: 0 }], &plug);
        // self.set_count
        let mut adapters:Vec<u128> = Vec::new();
        adapters.push(0);
        adapters.extend(self.adapters.iter().map(|adapter|adapter.rating as u128));
        adapters.push(self.device.rating as u128);
        // debug_print!("Values: {:?}",adapters);
        // AdapterBag::part2_helper(&mut HashMap::new(), adapters[0], 1, &*adapters)
        AdapterBag::all_chain_from_wall_helper(&*adapters,&mut HashMap::new())
    }
    // Recursive helper. Memoized via memo_table to make not insanely expensive.  Returns the number of
// ways to connect the adapter `prev_val` to the adapters in `&nums[start_idx..]`. We pass
// `start_idx` explicitly instead of the subslice itself, so that we can use `(prev_val,
// start_idx)` as a cheap key into the memo table.
    fn part2_helper(
        memo_table: &mut HashMap<(u32, usize), u32>,
        prev_val: u32,
        start_idx: usize,
        nums: &[u32],
    ) -> u32 {
        if let Some(res) = memo_table.get(&(prev_val, start_idx)) {
            // We've already computed this. Return the previous result.
            return *res;
        }

        // Alias for the slice of interest, for convenience.
        let slice = &nums[start_idx..nums.len()];

        // We should never be called with a zero-length slice (assuming sane original input).
        debug_assert_ne!(slice.len(), 0);

        // If the gap between the previous value and the first value in the slice is too large, then
        // there are no ways to arrange the slice.
        if slice[0] - prev_val > 3 {
            return 0;
        }
        // If there's only one value in the slice, then there's only one possibility.
        if slice.len() == 1 {
            return 1;
        }
        // Number of ways to arrange the slice if we include the first value of the slice.
        let ways_with_next =
            AdapterBag::part2_helper(memo_table, slice[0], start_idx + 1, nums);
        // Number of ways to arrange the slice if we *don't* include the first value of the slice.
        let ways_without_next = if slice[1] - prev_val > 3 {
            0
        } else {
            AdapterBag::part2_helper(memo_table, prev_val, start_idx + 1, nums)
        };
        let res = ways_with_next + ways_without_next;
        // Memoize recursive results, so that we don't need to recompute them later.
        memo_table.insert((prev_val, start_idx), res);
        res
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