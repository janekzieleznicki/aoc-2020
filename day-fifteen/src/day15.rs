use crate::numbers::{NumberGenerator,spoken_number};

mod numbers;

fn main() {

    let test_data: Vec<(&[u64],u64)> = vec![
        (&[0,3,6], 175594),
        (&[1,3,2], 2578),
        (&[2,1,3], 3544142),
        (&[1,2,3], 261214),
        (&[2,3,1], 6895259),
        (&[3,2,1], 18),
        (&[3,1,2], 362),
    ];
    test_data.into_iter().for_each(|(start, expected)|{
        let mut generator = NumberGenerator::from(start);
        assert_eq!(spoken_number(&mut generator, 30000000), expected);
    });

    let mut generator = NumberGenerator::from(vec![16,11,15,0,1,7].as_slice());
    println!("30000000th number spoken is {}", spoken_number(&mut generator, 30000000));
}
