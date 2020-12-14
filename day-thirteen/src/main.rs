#![feature(test)]

mod bus;

use crate::bus::*;

fn main() {
    // {
    //     let buses = from_str("1789,37,47,1889");
    //     println!("Earliest common: {:?}", earliest_v2(buses, |time| false));
    // }
    // {
    //     let buses = from_str("19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13");
    //     println!("Earliest common: {:?}", earliest_v2(buses, |time| false));
    // }
    {
        let buses = from_str("19,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,599,x,29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,x,x,x,x,x,23,x,x,x,x,x,x,x,761,x,x,x,x,x,x,x,x,x,41,x,x,13");
        println!("Earliest common: {:?}", earliest_v3(buses, |time| false));
    }
}
