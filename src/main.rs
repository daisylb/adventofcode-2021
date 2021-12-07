use std::env::args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    match &args().nth(1).unwrap()[..] {
        "1" => day1::day1(),
        "1a" => day1::day1a(),
        "2" => day2::day2(),
        "2a" => day2::day2a(),
        "3" => day3::day3(),
        "3a" => day3::day3a(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        "5a" => day5::day5a(),
        "6" => day6::day6(),
        _ => panic!("that day hasn't been added yet"),
    }
}
