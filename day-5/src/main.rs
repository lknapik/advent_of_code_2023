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
    
    println!("part 2: {}", part_2(lines));


}

#[derive(Debug)]
struct AlmanacMaps {
    dest_start: u64,
    source_start: u64,
    range: u64
}

fn translate_map( input: u64, maps: &Vec<AlmanacMaps> ) -> u64 {

    for map in maps {
        if input >= map.source_start && input < map.source_start + map.range {
            return map.dest_start + (input - map.source_start)
        }
    }
    
    input
}

fn part_1(lines: Lines<BufReader<File>>) -> u64 {

    let mut lines = lines;

    let mut maps_list: Vec<Vec<AlmanacMaps>> = Vec::new();

    let seeds = lines.next().unwrap().expect("line").split(": ").last().unwrap().split_whitespace().map(|seed| seed.parse().unwrap()).collect::<Vec<u64>>();

    for line in lines {
        let line = line.expect("line");
        if line == "" {
            continue;
        }

        if line.contains("map") {
            maps_list.push(Vec::new());
        } else {

            let data = line.split(" ").map(|num| num.parse().unwrap()).collect::<Vec<u64>>();
            maps_list.last_mut().unwrap().push(AlmanacMaps { dest_start: data[0], source_start: data[1], range: data[2]});
        }

    }

    let mut minimum: u64 = u64::MAX;

    for seed in seeds {
        let mut input = seed;
        for maps in &maps_list {
            input = translate_map(input, &maps)
        }

        if input < minimum {
            minimum = input;
        }
    }   


    minimum
}

fn part_2(lines: Lines<BufReader<File>>) -> u64 {
    let mut lines = lines;

    let mut maps_list: Vec<Vec<AlmanacMaps>> = Vec::new();

    let seeds: Vec<(u64, u64)> = lines.next().unwrap().expect("line").split(": ").last().unwrap().split_whitespace().map(|seed| seed.parse().unwrap()).collect::<Vec<u64>>().chunks(2).map(|w| (w[0], w[1])).collect();

    for line in lines {
        let line = line.expect("line");
        if line == "" {
            continue;
        }

        if line.contains("map") {
            maps_list.push(Vec::new());
        } else {

            let data = line.split(" ").map(|num| num.parse().unwrap()).collect::<Vec<u64>>();
            maps_list.last_mut().unwrap().push(AlmanacMaps { dest_start: data[0], source_start: data[1], range: data[2]});
        }

    }

    let mut minimum: u64 = u64::MAX;

    for seed_pair in seeds {
        for seed in seed_pair.0..seed_pair.0+seed_pair.1 {
            let mut input = seed;
            for maps in &maps_list {
                input = translate_map(input, &maps)
            }

            if input < minimum {
                minimum = input;
            }
        }   
    }
        
    minimum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_part_1() {
        let file = File::open("real.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_1(lines), 388071289);
    }

    //takes about 2 minutes to run
    // #[test]
    // fn real_part_2() {
    //     let file = File::open("real.csv")
    //     .expect("file not found");
    
    //     //create buffered reader to read line by line
    //     let buf_reader = BufReader::new(file);

    //     let lines  = buf_reader.lines();

    //     assert_eq!(part_2(lines), 84206669);
    // }

    #[test]
    fn test_part_1() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_1(lines), 35);
    }

    #[test]
    fn test_part_2() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_2(lines), 46);
    }
}