use std::str::FromStr;

mod input;

use crate::input::input;

fn main() {

    let mut i = 0;

    let mut num_array: Vec<u32> = Vec::<u32>::new();
    for string in input.split('\n') {

        let mut array: Vec<char> = Vec::<char>::new();
        for chr in string.chars() {
            if chr.is_numeric() {
                array.push(chr);
            }
        }
        num_array.push({
            let first_char = array.first().unwrap().to_string();
            let second_char = array.last().unwrap().to_string();

            let full = (first_char + second_char.as_str());
            let result = full.parse::<u32>().unwrap();

            result 
        });
        println!("{}: {} adds to {}", i, string, num_array[i]);
        i += 1;
    }
    let mut total = 0;
    for num in num_array.iter() {
        total = total + num;
    }
    println!("the total is {}", total);
}
