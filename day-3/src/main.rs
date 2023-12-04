use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct ReadWindow {
    buffer: Lines<BufReader<File>>,
    current_line_number: i32,
    previous: String,
    current: String,
    next: String
}

impl ReadWindow {
    fn get_next(&mut self) {
        
        let next_line = match self.buffer.next() {
            Some(line) => line.unwrap(),
            None => "".to_string()
        };

        self.previous = self.current.to_string();
        self.current = self.next.to_string();
        self.next = next_line;
        self.current_line_number += 1;
    }

    fn check_symbols(&self, part_number: u32, c_index: i32) -> (bool, Vec<i32>) {
        //get number length
        let number_len: i32 = (part_number.ilog10() + 1) as i32;
        
        let mut c_index = c_index as i32;
        if c_index == 0 {
            c_index = self.current.len() as i32;
        }
        let mut j: i32 = c_index as i32;

        let mut is_valid: bool = false;
        let mut gear_locations: Vec<i32> = Vec::new();

        //Should protect against reading before or after line bounds
        while j >= 0 && j >= (c_index-(number_len+1)) {
            let previous = self.previous.chars().nth(j as usize).unwrap_or('.');
            let current = self.current.chars().nth(j as usize).unwrap_or('.');
            let next = self.next.chars().nth(j as usize).unwrap_or('.');

            //Scanning each 3 lines from back to front, if not a . or number, but be symbol, set P/N to valid and break
            if !((previous.is_numeric() || previous == '.') && (current.is_numeric() || current == '.') && (next.is_numeric() || next == '.')) {
                if previous == '*' {
                    gear_locations.push((self.current_line_number - 1) * self.current.len() as i32 + j);

                } 
                if current == '*' {
                    gear_locations.push((self.current_line_number) * self.current.len() as i32 + j);

                } 
                if next == '*' {
                    gear_locations.push((self.current_line_number + 1) * self.current.len() as i32 + j);

                }
                
                is_valid = true;
                
            }

            j -= 1;
        }

    (is_valid, gear_locations)
    }   
}


#[derive(Debug, Copy, Clone)]
struct PartNumber {
    part_number: u32,
    valid: bool
}

#[derive(Debug)]
struct GearParts {
    location: i32, //absoute location, number of chars from beginning
    parts: Vec<PartNumber>,
    valid: bool
}

fn update_gears(gear_locations: Vec<i32>, all_gears: &mut Vec<GearParts>, part_number: &mut PartNumber) {
    for gear_loc in gear_locations {
        let mut new_gear: bool = true;

        for gear in &mut *all_gears {
            if part_number.valid && gear.location == gear_loc {
                gear.parts.push(*part_number);

                if gear.parts.len() == 2 {
                    gear.valid = true;
                } else {
                    gear.valid = false;
                }
                new_gear = false;
            }
        }

        if new_gear {
            let mut gear = GearParts { location: gear_loc, parts: Vec::new(), valid: false };
            gear.parts.push(*part_number);

            all_gears.push(gear);
        }
    }

    //all_gears
}


fn main() {
    //open file handle
    let file = File::open("real.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines = buf_reader.lines();

    let mut read_window = ReadWindow {
        buffer: lines,
        current_line_number: -2,
        previous: "".to_string(),
        current: "".to_string(),
        next: "".to_string()
    };

    read_window.get_next();
    read_window.get_next();


    let mut all_part_numbers: Vec<PartNumber> = Vec::new();
    //init an empty P/N
    all_part_numbers.push(PartNumber {part_number: 0, valid: false});

    let mut all_gears: Vec<GearParts> = Vec::new();

    while read_window.current.len() > 0 {

        //Read current line till you find a number
        for (c_index, c) in read_window.current.chars().enumerate() {
            //get last P/N in Vec
            let part_number = all_part_numbers.last_mut().unwrap();

            //We're either starting to, or currently reading a part number
            if c.is_numeric() {
                part_number.part_number = part_number.part_number * 10 + c.to_digit(10).unwrap();

                //if last index in line, must be end of part number
                if c_index == read_window.current.len() - 1 {
                    let gear_locations: Vec<i32>;
                    (part_number.valid, gear_locations) = read_window.check_symbols(part_number.part_number, (c_index + 1)as i32);

                    update_gears(gear_locations, &mut all_gears, part_number);
                
                    //Make a new part number since we're not currently reading one
                    all_part_numbers.push(PartNumber {part_number: 0, valid: false});
                }
                
            } else {
                
                if part_number.part_number != 0 {
                    //check if last part number was valid, protective if here just in case
                    if !part_number.valid {
                        let gear_locations: Vec<i32>;
                        (part_number.valid, gear_locations) = read_window.check_symbols(part_number.part_number, c_index as i32);

                        update_gears(gear_locations, &mut all_gears, part_number);
                    
                    }

                    //Make a new part number since we're not currently reading one
                    all_part_numbers.push(PartNumber {part_number: 0, valid: false});
                }
            }
        }

        read_window.get_next();
    }

    let mut sum = 0;
    let mut sum2 = 0;

    for pn in all_part_numbers {
        if pn.valid {
            sum += pn.part_number;
        }
    }

    for gear in all_gears {
        if gear.valid {
            sum2 += gear.parts[0].part_number * gear.parts[1].part_number;

        }
    }

    println!("part 1: {}", sum);

    println!("part 2: {}", sum2);

}