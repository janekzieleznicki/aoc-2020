use std::str::FromStr;
use itertools::Itertools;

use debug_print::{debug_print};

type ChildNode<T> = Option<Box<BTNode<T>>>;

#[derive(Debug, Eq, PartialEq)]
struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl BTNode<i32> {
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {
            op: op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

// fn btnode_parse_helper(strings: &[&str], parent: &mut ChildNode<i32>) -> BTNode<i32>{

// match parent {
//     None =>
//     Some(parent) =>
// }
// }
// impl FromStr for BTNode<i32> {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let split = s.split_ascii_whitespace().collect_vec();
//     }
// }

#[derive(Debug, Eq, PartialEq)]
enum Op<T> {
    Add,
    Mul,
    Val(T),
}

fn AddNode(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Add, l, r)
}

fn MulNode(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op::Mul, l, r)
}

fn ValNode(value: i32) -> BTNode<i32> {
    BTNode {
        left: None,
        right: None,
        op: Op::Val(value),
    }
}

struct BinaryTree<T> {
    head: Option<BTNode<T>>
}

impl BinaryTree<i32> {
    pub fn new(head: BTNode<i32>) -> Self {
        BinaryTree::<i32> { head: Some(head) }
    }
    pub fn collapse(node: &Box<BTNode<i32>>) -> i32 {
        let mut r: Option<i32> = None;
        let mut l: Option<i32> = None;

        if let Some(left) = &node.left {
            l = Some(BinaryTree::collapse(left));
        }

        if let Some(right) = &node.right {
            r = Some(BinaryTree::collapse(right));
        }

        let l = match l {
            Some(x) => x,
            None => 0
        };
        let r = match r {
            Some(x) => x,
            None => 0
        };

        match node.op {
            Op::Add => { l + r }
            Op::Mul => { l * r }
            Op::Val(x) => x
        }
    }
}

impl FromStr for BinaryTree<i32> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Oper<T> {
    Add,
    Mul,
    Val(T),
    Start,
}

pub fn eval_helper(s: &str) -> i32 {
    s.split_ascii_whitespace().fold((0, Oper::<i32>::Start), |(mut accum, mut oper), substr| {
        if substr.chars().all(|c| c.is_numeric()) {
            match oper {
                Oper::<i32>::Add => accum += substr.parse::<i32>().unwrap(),
                Oper::<i32>::Mul => accum *= substr.parse::<i32>().unwrap(),
                Oper::<i32>::Start => accum = substr.parse::<i32>().unwrap(),
                _ => panic!("Unexpected input")
            }
        } else {
            match substr {
                "+" => oper = Oper::<i32>::Add,
                "*" => oper = Oper::<i32>::Mul,
                _ => panic!("Unexpected input")
            }
        }
        (accum, oper)
    }).0
}

pub fn get_subexpression(s: &str) -> &str {
    let left_pos = s.find("(").unwrap();
    let mut depth = 0;
    let right_pos = s.chars().dropping(left_pos).position(|c|
        {
            // debug_print!("Depth: {}, char: {:?}",depth,c);
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            };
            depth == 0
        }
    ).unwrap();
    // debug_print!("Depth: {}, char: {:?}",depth,c);
    &s[left_pos + 1..left_pos + right_pos]
}

pub fn evaluate(s: &str) -> i64 {
    let mut ignore = 0;
    debug_print!("Evaluating expression: {:?}",s);
    let res = s.chars().enumerate().fold((0, Oper::<i32>::Start), |(mut accum, mut oper), (idx, substr)| {
        if ignore != 0 {
            ignore -= 1;
        } else if substr.is_numeric() {
            match oper {
                Oper::<i32>::Add => accum += (substr.to_digit(10).unwrap() as i64),
                Oper::<i32>::Mul => accum *= (substr.to_digit(10).unwrap() as i64),
                Oper::<i32>::Start => accum = (substr.to_digit(10).unwrap() as i64),
                _ => panic!("Unexpected input")
            }
        } else if substr.is_whitespace() {} else {
            match substr {
                '+' => oper = Oper::<i32>::Add,
                '*' => oper = Oper::<i32>::Mul,
                '(' => {
                    let subexpression = get_subexpression(&s[idx..]);
                    ignore = subexpression.len() + 2;
                    debug_print!("Found subexpression: {:?}\n",subexpression);
                    let subexpression = evaluate(subexpression);
                    match oper {
                        Oper::<i32>::Add => accum += subexpression,
                        Oper::<i32>::Mul => accum *= subexpression,
                        Oper::<i32>::Start => accum = subexpression,
                        _ => panic!("Unexpected input")
                    }
                }
                x => panic!("Unexpected input from {}: {} | ignore: {}", s, x, ignore)
            }
        }
        (accum, oper)
    }).0;
    debug_print!("| Res: {}\n",res);
    res
}

pub fn evaluate_part2(s: &str) -> i64 {
    let mut ignore = 0;
    debug_print!("Evaluating expression: {:?}",s);
    let res = s.chars().enumerate().fold((0, Oper::<i32>::Start), |(mut accum, mut oper), (idx, substr)| {
        if ignore != 0 {
            ignore -= 1;
        } else if substr.is_numeric() {
            match oper {
                // Oper::<i32>::Add => accum += (substr.to_digit(10).unwrap() as i64),
                Oper::<i32>::Mul => accum *= {
                    substr.to_digit(10).unwrap() as i64 },
                Oper::<i32>::Start => accum = (substr.to_digit(10).unwrap() as i64),
                _ => panic!("Unexpected input")
            }
        } else if substr.is_whitespace() {} else {
            match substr {
                '+' => {
                    let subexpression = &s[idx - 2..=idx + 2];
                    ignore = 2;
                    // debug_print!("Found subexpression: {:?}\n",subexpression);
                    let subexpression = evaluate(subexpression);
                    match oper {
                        Oper::<i32>::Add => accum += subexpression,
                        Oper::<i32>::Mul => accum *= subexpression,
                        Oper::<i32>::Start => accum = subexpression,
                        _ => panic!("Unexpected input")
                    }
                }
                '*' => oper = Oper::<i32>::Mul,
                '(' => {
                    let subexpression = get_subexpression(&s[idx..]);
                    ignore = subexpression.len() + 2;
                    // debug_print!("Found subexpression: {:?}\n",subexpression);
                    let subexpression = evaluate_part2(subexpression);
                    match oper {
                        Oper::<i32>::Add => accum += subexpression,
                        Oper::<i32>::Mul => accum *= subexpression,
                        Oper::<i32>::Start => accum = subexpression,
                        _ => panic!("Unexpected input")
                    }
                }
                x => panic!("Unexpected input from {}: {} | ignore: {}", s, x, ignore)
            }
        }
        (accum, oper)
    }).0;
    debug_print!("| Res: {}\n",res);
    res
}

#[cfg(test)]
mod tests {
    use crate::expressions::{AddNode, ValNode, MulNode, BinaryTree, BTNode, eval_helper, evaluate, get_subexpression, evaluate_part2};
    use crate::expressions::Op::Val;
    use std::str::FromStr;

    #[test]
    fn from_example() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let bt = BinaryTree::new(
            AddNode(
                MulNode(
                    AddNode(
                        MulNode(
                            AddNode(
                                ValNode(1),
                                ValNode(2)),
                            ValNode(3),
                        ),
                        ValNode(4),
                    ),
                    ValNode(5),
                ),
                ValNode(6),
            )
        );
        assert_eq!(BinaryTree::collapse(&Box::new(bt.head.expect("aaaa"))), 71)
    }

    #[test]
    fn from_str() {
        assert_eq!(AddNode(ValNode(1), ValNode(2)), AddNode(ValNode(1), ValNode(2)));
        // assert_eq!(BTNode::from_str("1 + 2").unwrap(), AddNode(ValNode(1), ValNode(2)));
    }

    #[test]
    fn evaluator_test() {
        assert_eq!(get_subexpression("2 * 3 + (4 * 5)"), "4 * 5");
        assert_eq!(get_subexpression("1 + (2 * 3) + (4 * (5 + 6))"), "2 * 3");
        assert_eq!(get_subexpression("+ (4 * (5 + 6))"), "4 * (5 + 6)");
        assert_eq!(eval_helper("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(evaluate("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn evaluator_part2() {
        // assert_eq!(eval_helper("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(evaluate_part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(evaluate_part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(evaluate_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(evaluate_part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }
}