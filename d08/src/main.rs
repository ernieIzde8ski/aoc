use std::{collections::HashMap, rc::Rc};

use common::read_input;

fn part_1() -> usize {
    let input = read_input!();
    let mut input = input.lines().filter(|line| !line.is_empty());

    let instructions = input
        .next()
        .expect("should exist")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            c => panic!("unexpected: '{c}'"),
        })
        .cycle();

    let mut nodes: HashMap<String, Rc<[String; 2]>> = HashMap::new();

    for line in input {
        let mut fragments = line.split_ascii_whitespace().filter_map(|s| {
            let s: String = s.chars().filter(char::is_ascii_alphabetic).collect();
            match s.is_empty() {
                true => None,
                false => Some(s),
            }
        });
        let key = fragments.next().expect("should exist");
        let left = fragments.next().expect("should exist");
        let right = fragments.next().expect("should exist");
        nodes.insert(key, Rc::new([left, right]));
    }

    let mut cur_node = "AAA";

    for (i, inst) in instructions.enumerate() {
        let new_node = &nodes.get(cur_node).expect("should exist")[inst];
        // println!("{cur_node}[{inst}] => {new_node} ({})",     i);
        if new_node == "ZZZ" {
            return i + 1;
        }
        cur_node = new_node;
    }

    panic!("This is impossible lmao")
}

fn part_2() -> usize {
    let input = read_input!();
    let mut input = input.lines().filter(|line| !line.is_empty());

    let instructions = input
        .next()
        .expect("should exist")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            c => panic!("unexpected: '{c}'"),
        })
        .cycle();

    let mut nodes: HashMap<String, Rc<[String; 2]>> = HashMap::new();

    for line in input {
        let mut fragments = line.split_ascii_whitespace().filter_map(|s| {
            let s: String = s.chars().filter(char::is_ascii_alphanumeric).collect();
            match s.is_empty() {
                true => None,
                false => Some(s),
            }
        });
        let key = fragments.next().expect("should exist");
        let left = fragments.next().expect("should exist");
        let right = fragments.next().expect("should exist");
        nodes.insert(key, Rc::new([left, right]));
    }

    let starting_nodes: Vec<&str> = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k as &str)
        .collect();

    let mut multiples: Vec<usize> = Vec::new();

    for mut node in starting_nodes {
        let instructions = instructions.clone().enumerate();
        for (i, inst) in instructions {
            let new_node = &nodes.get(node).expect("should exist")[inst];
            if new_node.ends_with('Z') {
                multiples.push(i + 1);
                break;
            }
            node = new_node;
        }
    }
    let gcd = |a: usize, b: usize| {
        let mut a = a;
        let mut b = b;
        while b > 0 {
            (a, b) = (b, a % b);
        }
        a
    };
    let lcm = |a, b| a * b / gcd(a, b);

    multiples.into_iter().fold(1, lcm)
}

fn main() {
    let _input = read_input!();

    println!("{}, {}", part_1(), part_2());
}
