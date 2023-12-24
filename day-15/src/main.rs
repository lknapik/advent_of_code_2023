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

fn calc_hash(data: &str) -> u32 {

    let mut hash_val: u32 = 0;
    for c in data.chars() {
        if c.is_ascii() {
            hash_val += (c as u8) as u32;
            hash_val *= 17;
            hash_val = hash_val.rem_euclid(256);
        } else {
            panic!("Not ascii char");
        }
    }

    hash_val
}

fn part_1(mut lines: Lines<BufReader<File>>) -> u32 {

    let raw_data = lines.next().unwrap().unwrap();

    let data_vec = raw_data.split(",").collect::<Vec<&str>>();


    let mut sum: u32 = 0;
    for data in data_vec {
        sum += calc_hash(data);
    }
    sum
}

#[derive(Debug)]
struct Lens {
    id: String,
    length: u32
}

#[derive(Debug)]
struct LensBox { 
    index: u32,
    lenses: Vec<Lens>
}

fn update_lenses(new_lens: Lens, all_lenses: &mut Vec<LensBox>) {
    let box_id = calc_hash(new_lens.id.as_str());

    for lens_box in &mut *all_lenses {
        if lens_box.index == box_id {
            for lens in &mut lens_box.lenses {
                if lens.id == new_lens.id {
                    lens.length = new_lens.length;
                    return
                }
            }

            lens_box.lenses.push(new_lens);
            return
        }
    }

    let mut new_box = LensBox { index: box_id, lenses: Vec::new() };
    new_box.lenses.push(new_lens);

    all_lenses.push(new_box);
}

fn delete_lens(lens_id: &str, all_lenses: &mut Vec<LensBox>) {
    let box_id = calc_hash(lens_id);

    for lens_box in all_lenses {
        if lens_box.index == box_id {
            for (index, lens) in lens_box.lenses.iter_mut().enumerate() {
                if lens.id == lens_id {
                    lens_box.lenses.remove(index);
                    return
                }
            }
        }
    }
}

fn part_2(mut lines: Lines<BufReader<File>>) -> u32 {
    let raw_data = lines.next().unwrap().unwrap();

    let data_vec = raw_data.split(",").collect::<Vec<&str>>();

    let mut all_lenses: Vec<LensBox> = Vec::new();

    for data in data_vec {
        if data.contains("=") {
            let v = data.split("=").collect::<Vec<_>>();
            let parsed_data: (String, u32) = (v[0].to_string(), v[1].parse().unwrap());

            let lens = Lens { id: parsed_data.0, length: parsed_data.1 };

            update_lenses(lens, &mut all_lenses);

        } else if data.contains("-") {
            let lens_id = &data[0..data.len() - 1];

            delete_lens(lens_id, &mut all_lenses);
            
        } else {
            panic!("No operation...");
        }
    }

    let mut sum: u32 = 0;

    for lens_box in all_lenses {
        for (index, lens) in lens_box.lenses.iter().enumerate() {
            sum += ( lens_box.index + 1 ) *  ( index as u32 + 1 ) * lens.length;
        }
    }

    sum
}