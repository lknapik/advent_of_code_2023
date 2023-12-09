use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::fs::File;

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

fn get_next_number(seq: Vec<i32>) -> i32 {

    if seq.iter().max() == seq.iter().min() {
        return seq[0]
    }

    let next_seq = seq.as_slice().windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    return get_next_number(next_seq) + seq[seq.len()-1]
}

fn get_previous_number(seq: Vec<i32>) -> i32 {

    if seq.iter().max() == seq.iter().min() {
        return seq[0]
    }

    let next_seq = seq.as_slice().windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>();
    return seq[0] - get_previous_number(next_seq)

}
fn part_1(lines: Lines<BufReader<File>>) -> i32 {

    let mut sum: i32 = 0;

    for line in lines {
        let line = line.expect("line");

        let seq = line.split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<i32>>();
        let foo = get_next_number(seq);
        sum += foo;
    }
    sum
}

fn part_2(lines: Lines<BufReader<File>>) -> i32 {

    let mut sum: i32 = 0;

    for line in lines {
        let line = line.expect("line");

        let seq = line.split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<i32>>();
        let foo = get_previous_number(seq);
        sum += foo;
    }
    sum
}