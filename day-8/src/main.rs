use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::fs::File;

use std::collections::HashMap;

fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines  = buf_reader.lines();

    println!("part 1: {}", part_1(lines));

    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines  = buf_reader.lines();
    
    println!("part 2: {}" ,part_2(lines));

}

//Stolen from https://github.com/TheAlgorithms
/*--------------------------------------------------*/
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
/*--------------------------------------------------*/

fn part_1(lines: Lines<BufReader<File>>) -> u32 {

    let mut lines = lines;

    let instructions = lines.next().expect("line").unwrap();
    lines.next(); //Blank line

    let mut hash_map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let line = line.expect("line");

        let tmp = line.replace(&['(', ')', ',', '='], "");
        let w = tmp.split_whitespace().collect::<Vec<&str>>();
        hash_map.insert(w[0].to_string(), (w[1].to_string(), w[2].to_string()));

    }

    let mut node: String = "AAA".to_string();
    let mut counter: usize = 0;

    while node != "ZZZ".to_string() {
        if instructions.chars().nth(counter.rem_euclid(instructions.len())).unwrap() == 'L' {
            node = hash_map.get(&node).unwrap().0.clone();
        } else if instructions.chars().nth(counter.rem_euclid(instructions.len())).unwrap() == 'R' {
            node = hash_map.get(&node).unwrap().1.clone();
        }

        counter += 1;
    }

    counter as u32
}

#[derive(Debug)]
struct Node { 
    _start: String,
    current: String,
    z_index: usize,
}

fn part_2(lines: Lines<BufReader<File>>) -> usize {
    let mut lines = lines;

    let instructions = lines.next().expect("line").unwrap();
    lines.next(); //Blank line

    let mut hash_map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let line = line.expect("line");

        let tmp = line.replace(&['(', ')', ',', '='], "");
        let w = tmp.split_whitespace().collect::<Vec<&str>>();
        hash_map.insert(w[0].to_string(), (w[1].to_string(), w[2].to_string()));

    }

    let mut nodes: Vec<Node> = Vec::new();

    for node in &hash_map {
        if node.0.chars().last().unwrap() == 'A' {
            nodes.push(Node { _start: node.0.clone(), current: node.0.clone(), z_index: 0});
        }
    }

    let mut dir: char = 'L';
    let mut not_done: bool = true;
    let mut counter: usize = 0;

    while not_done {


        if instructions.chars().nth(counter.rem_euclid(instructions.len())).unwrap() == 'L' {
            dir = 'L';
        } else if instructions.chars().nth(counter.rem_euclid(instructions.len())).unwrap() == 'R' {
            dir = 'R';
        }

        for node in &mut nodes {
            if dir == 'L' {
                node.current = hash_map.get(&node.current).unwrap().0.clone();
            } else if dir == 'R' {
                node.current = hash_map.get(&node.current).unwrap().1.clone();
            }


        }

        counter += 1;

        for node in &mut nodes {
            if node.z_index == 0 && node.current.chars().last().unwrap() == 'Z' {
                node.z_index = counter;
            }
        }

        let mut done: bool = true;

        for node in &nodes {
            if node.z_index == 0 {
                done = false;
            }
        }

        if done { not_done = false };
    }

    let mut loop_counters: Vec<usize> = Vec::new();
    for node in &nodes {
        loop_counters.push(node.z_index);
    }

    lcm(&loop_counters)
}