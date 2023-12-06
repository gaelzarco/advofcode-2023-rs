use std::{collections::HashMap, fs};

fn dayone_one() {
    let contents = fs::read_to_string("./src/inputs/day1.txt").unwrap();
    let mut code_vec: Vec<String> = vec![];
    let mut sum_vec: Vec<String> = vec![];
    let mut total_sum: i32 = 0;

    for code in contents.lines() {
        match code.parse::<String>().map(|i| code_vec.push(i)) {
            Ok(_) => continue,
            Err(_) => println!("Could not parse code!"),
        }
    }

    for i in 0..code_vec.len() {
        let mut split_code: Vec<String> = vec![];
        let mut code_total: Vec<String> = vec![];
        code_vec[i].chars().for_each(|c| {
            if c.is_digit(10) {
                split_code.push(c.to_string());
            }
        });

        code_total.push(split_code[0].clone());
        code_total.push(split_code[split_code.len() - 1].clone());

        sum_vec.push(code_total.join(""));
    }

    for sum in sum_vec {
        match sum.parse::<i32>() {
            Ok(parsed_sum) => total_sum = total_sum + parsed_sum,
            Err(_) => {
                println!("Could not parse to i32");
                return;
            }
        }
    }

    println!("total sum = {}", total_sum);
}

fn dayone_two() {
    let input = fs::read_to_string("./src/inputs/day1.txt").unwrap();
    let mut text = input;
    let mut chars: Vec<char> = text.chars().collect();
    let num_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for i in 0..10 {
        let mut loc = text.find(num_words[i]);
        while loc.is_some() {
            chars[loc.unwrap() + 1] = char::from_digit(i as u32, 10).unwrap();
            text = chars.clone().into_iter().collect();
            loc = text.find(num_words[i])
        }
    }
    let full: Vec<&str> = text.split("\n").collect();
    //split string into chars, just take the chars that are digits, multiply first by 10, add to last
    let mut total: u32 = 0;
    for input in full.iter() {
        let nums: Vec<u32> = input
            .chars()
            .filter(|&x| '0' <= x && '9' >= x)
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        if !nums.is_empty() {
            total += nums[0] * 10 + nums[nums.len() - 1];
        }
    }

    println!("total = {}", total);
}

fn daytwo_one() {
    let text = fs::read_to_string("./src/inputs/day2.txt").expect("Could not read file");

    let _max_hash: HashMap<&str, u32> = HashMap::from([("r", 12), ("g", 13), ("b", 14)]);

    for word in text.split(",") {
        let split_word: Vec<_> = word.trim().split(" ").collect();

        println!("{:?}", split_word);
    }
}

fn main() {
    dayone_one();
    dayone_two();
    daytwo_one();
}
