mod input;

use crate::input::INPUT;
use colored::Colorize;

fn solve_first_problem() {
    let mut i = 0;

    let mut num_array: Vec<u32> = Vec::<u32>::new();
    for string in INPUT.split('\n') {

        let mut array: Vec<char> = Vec::<char>::new();
        for chr in string.chars() {
            if chr.is_numeric() {
                array.push(chr);
            }
        }
        num_array.push({
            let first_char = array.first().unwrap().to_string();
            let second_char = array.last().unwrap().to_string();

            let full = first_char + second_char.as_str();
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

fn solve_second_problem() {
    let mut i = 0;

    let input = "two1nine
jcb82eightwond
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    let numbers :[(&str, u8); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let mut num_array: Vec<u32> = Vec::<u32>::new();
    for string in INPUT.split('\n') {
        println!("=====");

        let mut new_string: String = string.to_string();

        let mut array: Vec<(u8, usize)> = Vec::<(u8, usize)>::new();
        let mut errors: Vec<((&str, u8), usize)> = Vec::<((&str, u8), usize)>::new();
        for num in numbers {
            if new_string.contains(num.0) {
                // println!{"{} contains {}", new_string, num.0};
                // let index = new_string.find(num.0).unwrap();
                let index: Vec<_> = new_string.match_indices(num.0).collect();
                for i in index {
                    errors.push((num, i.0));
                }
                // errors.push((num, index));
            } else {
                // println!{"{} doesn't contain {}", new_string, num.0};
            }
        }
        errors.sort_by(|a, b| a.1.cmp(&b.1));

        for error in errors {
            array.push((error.0.1, error.1));
        }

        for (i, chr) in new_string.chars().enumerate() {
            if chr.is_numeric() {
                array.push((chr as u8 - 48, i));
            }
        }

        array.sort_by(|a, b| a.1.cmp(&b.1));

        for (i, x) in array.clone().iter().enumerate() {
            println!("i: {}, x: {} ", i, x.0);
        }

        // println!("{:#?}", array);
        
        num_array.push({
            let first_char = array.first().unwrap().0.to_string();
            let second_char = array.last().unwrap().0.to_string();
            println!("first: {}, last: {}", first_char, second_char);

            let full = first_char + second_char.as_str();

            // println!("{}", full);
            let result = full.parse::<u8>().unwrap();

            result as u32
        });
        println!("{}: {} gives {}", i, new_string.green(), num_array[i].to_string().blue());

        i += 1;
    }
    let mut total = 0;
    for num in num_array.iter() {
        total = total + num;
        // println!("the total is {}, num: {}", total.to_string().red(), num.to_string().blue());
        // println!("num: {}, total: {}", num, total);
    }
    println!("the total is {}", total);
}

fn main() {
    solve_first_problem();
    // solve_second_problem();
}
