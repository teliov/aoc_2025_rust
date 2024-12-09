pub mod days;
pub mod util;

use std::env;
use crate::util::read_file;


fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() >= 3, "Usage: {} <day> <part> <filename>", args[0]);

    let day_number = &args[1].parse::<i32>().unwrap();
    let part = &args[2].parse::<i32>().unwrap();
    let filename: &String = &args[3];

    let file_contents = read_file(filename);

    let day = crate::days::Day::from(*day_number);
    let result = if *part == 1 {
        day.solve_part_one(&file_contents)
    } else {
        day.solve_part_two(&file_contents)
    };



    println!("Result for day {day} part {part} is {result}")
}
