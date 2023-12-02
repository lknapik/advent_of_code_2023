use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct Game {
    id: u32,
    //count of cubes
    red: u32,
    green: u32,
    blue: u32
}

fn main() {
    //open file handle
    let file = File::open("test.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines  = buf_reader.lines();
    
    let mut sum = 0;
    let mut sum_part2 = 0;

    //for every line in the file
    for line in lines {
        //ensure the line was read correctly
        let line = line.expect("Unable to read line");

        let collection = line.split(":").collect::<Vec<&str>>();

        let id: u32 = collection[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

        let mut cur_game = Game {
            id: id,
            red: 0,
            green: 0,
            blue: 0
        };

        let rounds = collection[1].split(";").collect::<Vec<&str>>();

        for round in rounds {
            let colors = round.split(", ").collect::<Vec<&str>>();

            for color in colors {
                
                let cube = color.trim().split(" ").collect::<Vec<&str>>();
                
                if cube[1] == "red" {

                    if cur_game.red < cube[0].parse::<u32>().unwrap() {
                        cur_game.red = cube[0].parse::<u32>().unwrap();
                    }

                } else if cube[1] == "green" {

                    if cur_game.green < cube[0].parse::<u32>().unwrap() {
                        cur_game.green = cube[0].parse::<u32>().unwrap();
                    }

                } else if cube[1] == "blue" {

                    if cur_game.blue < cube[0].parse::<u32>().unwrap() {
                        cur_game.blue = cube[0].parse::<u32>().unwrap();
                    }

                }
            }
        }

        if cur_game.red <= 12 && cur_game.green <= 13 && cur_game.blue <= 14 {
            sum += cur_game.id;
        }

        sum_part2 += cur_game.red * cur_game.green * cur_game.blue;
    
    }

    println!("part 1: {}", sum);

    println!("part 2: {}", sum_part2);

}
