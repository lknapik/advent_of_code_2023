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

fn part_1(lines: Lines<BufReader<File>>) -> u32 {

    let mut lines = lines;
    let times = lines.next().expect("line").unwrap().split(":").last().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u32>>();
    let distances = lines.next().expect("line").unwrap().split(":").last().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

    let mut total = 1;
    for race in 0..times.len() {
        let mut sum = 0;
        for speed in 1..times[race] {
            if speed*(times[race]-speed) > distances[race] {
                sum += 1;
            }
        }

        total *= sum;
    }

    total
}

fn part_2(lines: Lines<BufReader<File>>) -> u32 {

    let mut lines = lines;
    let time = lines.next().expect("line").unwrap().split(":").last().unwrap().trim().split_whitespace().fold("".to_string(), |a, b| a + b).parse::<u64>().unwrap();
    let distance = lines.next().expect("line").unwrap().split(":").last().unwrap().trim().split_whitespace().fold("".to_string(), |a, b| a + b).parse::<u64>().unwrap();
    
    let mut sum: u32 = 0;

    for speed in 1..time {
        if speed*(time-speed) > distance {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_1(lines), 288);
    }

    #[test]
    fn test_part_2() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_2(lines), 71503);
    }
}
