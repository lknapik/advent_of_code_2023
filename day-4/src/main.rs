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


fn part_1(lines: Lines<BufReader<File>>) -> u32{
    let mut sum = 0;

    for line in lines {
        let mut card_points = 0;

        let line = line.expect("Parsed line");
        let numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();

        let winning_numbers = numbers[0].trim().split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();
        let your_numbers = numbers[1].trim().split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();

        for num in your_numbers {
            if winning_numbers.contains(&num) {
                if card_points == 0 {
                    card_points = 1;
                } else {
                    card_points *= 2;
                }
            }
        }
        sum += card_points;
    }
    
    sum
}

struct Card {
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

fn count_cards(all_cards: &Vec<Card>, index: usize) -> u32 {
    if index >= all_cards.len() {
        println!("bottomed out");
        return 0
    }

    let mut card_total: u32 = 0;
    let mut recurse_cards: usize = 0;
    
    let current_card = &all_cards[index];

    for num in &current_card.your_numbers {
        if current_card.winning_numbers.contains(num) {
            recurse_cards += 1;
            card_total += 1;
            card_total += count_cards(all_cards, index+recurse_cards);
        }
    }
    card_total
}

fn part_2(lines: Lines<BufReader<File>>) -> u32 {
    
    let mut all_cards: Vec<Card> = Vec::new();

    for line in lines {
        let line = line.expect("error reading");

        let numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();

        let winning_numbers = numbers[0].trim().split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();
        let your_numbers = numbers[1].trim().split_whitespace().map(|num| num.parse().unwrap()).collect::<Vec<u32>>();

        all_cards.push(Card {winning_numbers: winning_numbers, your_numbers: your_numbers});

    }

    let mut sum = 0;

    for index in 0..all_cards.len() {
        sum += 1;
        sum += count_cards(&all_cards, index);
    }

    sum
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

        assert_eq!(part_1(lines), 26346);
    }

    #[test]
    fn real_part_2() {
        let file = File::open("real.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_2(lines), 8467762);
    }

    #[test]
    fn test_part_1() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_1(lines), 13);
    }

    #[test]
    fn test_part_2() {
        let file = File::open("test.csv")
        .expect("file not found");
    
        //create buffered reader to read line by line
        let buf_reader = BufReader::new(file);

        let lines  = buf_reader.lines();

        assert_eq!(part_2(lines), 30);
    }
}