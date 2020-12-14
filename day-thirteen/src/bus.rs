#![feature(test)]

use itertools::{Itertools, sorted};
use num::integer::{lcm, gcd_lcm, gcd};
use reduce::Reduce;
use u128 as BusType;
use std::ops::RangeFrom;

fn is_multiple(left: BusType, right: BusType) -> bool {
    left % right == 0
}

pub fn from_str(s: &str) -> Vec<(usize, BusType)> {
    s.split(',').enumerate().filter_map(|(index, string)|
        match string {
            "x" => None,
            _ => Some((index, string.parse().unwrap()))
        }
    ).collect_vec()
}

pub fn earliest_v3(mut buses: Vec<(usize, BusType)>, stop_at: fn(BusType) -> bool) -> (BusType, usize) {
    buses.sort_by_key(|(_, bus)| *bus);
    buses.into_iter().fold((0, 1), |(start_time, increment), (index, bus)| {
        RangeFrom { start: start_time }
            .step_by(increment)
            .find_map(|time|
                match (time + index as BusType) % bus {
                    0 => Some((time, lcm(increment, bus as usize))),
                    _ => None
                })
            .unwrap()
    })
}

pub fn earliest_v3_rev(mut buses: Vec<(usize, BusType)>, stop_at: fn(BusType) -> bool) -> (BusType, usize) {
    buses.sort_by_key(|(_, bus)| *bus);
    buses.into_iter().rev().fold((0, 1), |(start_time, increment), (index, bus)| {
        RangeFrom { start: start_time }
            .step_by(increment)
            .find_map(|time|
                match (time + index as BusType) % bus {
                    0 => Some((time, lcm(increment, bus as usize))),
                    _ => None
                })
            .unwrap()
    })
}

pub fn earliest_v2(mut buses: Vec<(usize, BusType)>, stop_at: fn(BusType) -> bool) -> (BusType, usize) {
    buses.sort_by_key(|(_, bus)| *bus);
    let mut iter_count = 0;
    let (time, incr) = buses.into_iter().rev().fold((0, 1), |(start_time, increment), (index, bus)| {
        for i in 0.. {
            iter_count += 1;
            if (start_time + (increment as BusType * i) + index as BusType) % bus == 0 {
                return (start_time + (increment as BusType * i), lcm(increment, bus as usize));
            }
        }
        (0, 0)
    });
    (time, iter_count)
}

pub fn earliest(buses: Vec<(usize, BusType)>, stop_at: fn(BusType) -> bool) -> (BusType, usize) {
    let mut iter_count = 0;
    let mut time = 0;
    let highest_bus = buses.iter().map(|(index, bus)| bus).max().unwrap();
    let lowest_bus = buses.iter().map(|(index, bus)| bus).max().unwrap();
    let (start_time, inc) = buses.iter().find(|(_, bus)| bus == highest_bus).unwrap();
    time = inc - *start_time as BusType;
    loop {
        iter_count += 1;
        time += inc;
        if buses.iter().map(|(index, bus)| ((time + *index as BusType) % bus)).all(|x| x == 0) {
            return (time, iter_count);
        } else if stop_at(time) {
            return (time, iter_count);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bus::{is_multiple, from_str, earliest, earliest_v2, earliest_v3, earliest_v3_rev};
    use num::integer::lcm;

    extern crate test;

    use test::{Bencher, black_box};

    #[test]
    fn test_ex() {
        assert!(is_multiple(944, 59));
    }

    #[test]
    fn earliest_departure() {
        let buses = vec![7, 13, 59, 31, 19];
        let departure = 939;
        let res = (departure..u64::MAX).find_map(|dep|
            buses.iter().find_map(|&bus|
                if dep % bus == 0 {
                    Some((dep, bus))
                } else { None }
            )
        ).unwrap();
        assert_eq!(res, (944, 59));
        assert_eq!((res.0 - departure) * res.1, 295);
    }

    #[test]
    fn earliest_departure_real() {
        let buses = vec![19, 37, 599, 29, 17, 23, 761, 41, 13];
        let departure = 1009310;
        let res = (departure..u64::MAX).find_map(|dep|
            buses.iter().find_map(|&bus|
                if dep % bus == 0 {
                    Some((dep, bus))
                } else { None }
            )
        ).unwrap();
        assert_eq!(res, (1009315, 599));
        assert_eq!((res.0 - departure) * res.1, 2995);
    }

    #[test]
    fn part_two() {
        {
            let buses = from_str("17,x,13,19");
            assert_eq!(earliest(buses.clone(), |time| time > 3900).0, 3417);
            assert_eq!(earliest(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
            assert_eq!(earliest_v3(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
        }
        {
            let buses = from_str("67,7,59,61");
            assert_eq!(earliest(buses.clone(), |time| time > 754018 + 1).0, 754018);
            assert_eq!(earliest(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
            assert_eq!(earliest_v3(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
        }
        {
            let buses = from_str("67,x,7,59,61");
            assert_eq!(earliest(buses.clone(), |time| time > 779210 + 1).0, 779210);
            assert_eq!(earliest(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
        }
        {
            let buses = from_str("67,7,x,59,61");
            assert_eq!(earliest(buses.clone(), |time| time > 1261476 + 1).0, 1261476);
            assert_eq!(earliest(buses.clone(), |_| false).0, earliest_v2(buses.clone(), |_| false).0);
        }
        {
            let buses = from_str("1789,37,47,1889");
            assert_eq!(earliest(buses.clone(), |time| time > 1202161486 + 1).0, 1202161486);
            assert_eq!(earliest(buses.clone(), |time| time > 1202161486 + 1).0, earliest_v2(buses.clone(), |time| time > 1202161486 + 1).0);
        }
        let input = r#"19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13"#;
        let buses = from_str(input);
        let inc = buses.iter().map(|(index, bus)| bus).fold(1, |folded, reminder| lcm(folded, *reminder));
        // assert_eq!(&inc, buses.iter().map(|(index, bus)| bus).max().unwrap());
    }

    #[bench]
    fn bench_earliest(b: &mut Bencher) {
        let buses = from_str("1789,37,47,1889");
        b.iter(|| {
            let n = test::black_box(earliest(buses.clone(), |_| false).0);
        });
    }

    #[bench]
    fn bench_earliest_v2(b: &mut Bencher) {
        let buses = from_str("1789,37,47,1889");
        b.iter(|| {
            let n = test::black_box(earliest_v2(buses.clone(), |_| false).0);
        });
    }

    #[bench]
    fn bench_earliest_v3(b: &mut Bencher) {
        let buses = from_str("1789,37,47,1889");
        b.iter(|| {
            let n = test::black_box(earliest_v3(buses.clone(), |_| false).0);
        });
    }

    #[bench]
    fn bench_input_v2(b: &mut Bencher) {
        let buses = from_str("19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13");
        b.iter(|| {
            let n = test::black_box(earliest_v2(buses.clone(), |_| false).0);
        });
    }

    #[bench]
    fn bench_input_v3(b: &mut Bencher) {
        let buses = from_str("19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13");
        b.iter(|| {
            let n = test::black_box(earliest_v3(buses.clone(), |_| false).0);
        });
    }

    #[bench]
    fn bench_input_v3_rev(b: &mut Bencher) {
        let buses = from_str("19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13");
        b.iter(|| {
            let n = test::black_box(earliest_v3_rev(buses.clone(), |_| false).0);
        });
    }
}