use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let _N: usize = it.next().unwrap().parse().unwrap();
    let num_string: String = it.next().unwrap().to_string();
    let nums_char: Vec<char> = num_string.chars().collect();

    let mut sum: usize = 0;

    for i in nums_char {
        sum += i.to_digit(10).unwrap() as usize;
    }

    println!("{}", sum);

}