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
    let file = File::open("test.csv")
        .expect("file not found");
    
    //create buffered reader to read line by line
    let buf_reader = BufReader::new(file);

    let lines  = buf_reader.lines();
    
    println!("part 2: {}" ,part_2(lines));

}


fn part_1(lines: Lines<BufReader<File>>) -> usize {

    let mut full_chart: Vec<Vec<char>> = Vec::new();

    for line in lines {
        let line = line.expect("line");

        full_chart.push(line.chars().collect::<Vec<char>>());
    }

    let mut coords: Vec<(usize, usize)> = Vec::new(); 
    let line_len = full_chart[0].len();
    let mut find_s: bool = false;
    let mut coord_s: (usize, usize) = (0, 0);

    while !find_s {
        if full_chart[coord_s.1][coord_s.0] == 'S' {
            coords.push(coord_s);
            find_s = true;
            break;
        }

        coord_s.0 += 1;

        if coord_s.0 >= line_len {
            coord_s.0 = 0;
            coord_s.1 += 1;
        }
    }
    //look up
    if ['|', '7', 'F'].contains(&full_chart[coord_s.1 - 1][coord_s.0]) {
        coords.push((coord_s.0, coord_s.1 - 1))

    //look down
    } else if ['|', 'J', 'L'].contains(&full_chart[coord_s.1 + 1][coord_s.0]) {
        coords.push((coord_s.0, coord_s.1 + 1))

    //look left
    } else if ['-', 'F', 'L'].contains(&full_chart[coord_s.1][coord_s.0 - 1]) {
        coords.push((coord_s.0 - 1, coord_s.1))

    //look right
    } else if ['-', 'J', '7'].contains(&full_chart[coord_s.1][coord_s.0 + 1]) {
        coords.push((coord_s.0 + 1, coord_s.1))

    } else {
        panic!("no")
    }


    let mut find_loop: bool = false;

    while !find_loop {
        let last_coord: (usize, usize) = coords[coords.len()-2];
        let cur_coord: (usize, usize) = coords[coords.len()-1];
        let x_diff = cur_coord.0 as i32 - last_coord.0 as i32;
        let y_diff = cur_coord.1 as i32 - last_coord.1 as i32;

        match full_chart[cur_coord.1][cur_coord.0] {
            '|' => {
                //moved up, go up
                if y_diff == -1 {
                    coords.push((cur_coord.0, cur_coord.1 - 1));

                //moved down, go down
                } else if y_diff == 1 {
                    coords.push((cur_coord.0, cur_coord.1 + 1));

                }
            }
            '-' => {
                //moved left, go left
                if x_diff == -1 {
                    coords.push((cur_coord.0 - 1, cur_coord.1));

                //moved right, go right
                } else if x_diff == 1 {
                    coords.push((cur_coord.0 + 1, cur_coord.1));

                }
            }
            'L' => { 
                //moved down, go right
                if y_diff == 1 {
                    coords.push((cur_coord.0 + 1, cur_coord.1));
                //moved left, go up
                } else if x_diff == -1 {
                    coords.push((cur_coord.0, cur_coord.1 - 1));
                }
            }
            'J' => {
                //moved down, go left
                if y_diff == 1 {
                    coords.push((cur_coord.0 - 1, cur_coord.1));
                //moved right, go up
                } else if x_diff == 1 {
                    coords.push((cur_coord.0, cur_coord.1 - 1));       
                }

            }
            '7' => {
                //moved right, go down
                if x_diff == 1 {
                    coords.push((cur_coord.0, cur_coord.1 + 1));
                //moved up, go left
                } else if y_diff == -1 {
                    coords.push((cur_coord.0 - 1, cur_coord.1));
                }
            }
            'F' => {
                //moved left, go down
                if x_diff == -1 {
                    coords.push((cur_coord.0, cur_coord.1 + 1));
                //moved up, go right
                } else if y_diff == -1 {
                    coords.push((cur_coord.0 + 1, cur_coord.1));
                }
            }
            'S' => {find_loop = true; break},
            _ => panic!("missed symbol")
        }

    }

    (coords.len()-1)/2
}

fn part_2(lines: Lines<BufReader<File>>) -> u32 {
    0
}