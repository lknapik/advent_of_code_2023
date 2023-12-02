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
    part_1(lines);

    //open file handle
    let file = File::open("real.csv")
    .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines  = buf_reader.lines();

    part_2(lines);

}

fn part_1(lines: Lines<BufReader<File>>) {

    let mut sum: u32 = 0;

    //for every line in the file
    for line in lines {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");
        
        let mut first_char: char = '0';
        let mut last_char: char = '0';

        //Read from front till first numeric
        for c in line.chars() {
            if c.is_numeric() {
                first_char = c;
                break;
            }
        }

        //Read from back till 'first' numeric
        for c in line.chars().rev() {
            if c.is_numeric() {
                last_char = c;
                break;
            }
        }

        sum += first_char.to_digit(10).unwrap() * 10 + last_char.to_digit(10).unwrap();
    }

    println!("part 1: {}", sum);
}

fn part_2(lines: Lines<BufReader<File>>) {
    #[derive(Debug)]
    struct HighLowStruct {
        first_number: u8,
        first_index: i32,
        last_number: u8,
        last_index: i32,
    }

    let mut sum: u32 = 0;

    let grep_strings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];

    //for every line in the file
    for line in lines {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        let mut index_holder = HighLowStruct {
            first_number: 0,
            first_index: 0,
            last_number: 0,
            last_index: 0
        };

        //Loop over every way a number can be represented
        for (grep_index,grep, ) in grep_strings.iter().enumerate() {

            let matches: Vec<(usize, &str)> = line.match_indices(grep).collect();

            if matches.len() > 0 {

                //Shift for 0 index and adjust for numerics
                let mut number = ( grep_index + 1 ) as u8;
                if number > 9 {
                    number -= 9;
                }

                //For every match in the string, update the index
                for (index, _) in matches {
                    let index: i32 = index as i32; //This could be cleaner

                    //since 0 isn't valid, this must be the first run, initialize with values
                    if index_holder.first_number == 0 {

                        index_holder.first_number = number;
                        index_holder.first_index = index;
                        index_holder.last_number = number;
                        index_holder.last_index = index;
                        
                    } else if index_holder.first_index > index {

                        index_holder.first_number = number;
                        index_holder.first_index = index;

                    } else if index_holder.last_index < index {

                        index_holder.last_number = number;
                        index_holder.last_index = index;

                    }
                }
            }
        }

        sum += (index_holder.first_number * 10 + index_holder.last_number) as u32;
    }
    println!("part 2: {}", sum);

}