use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let nums: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (idx, &max_val) = nums
        .iter()
        .enumerate()
        .max_by_key(|&(_, v)| v)
        .unwrap();


    println!("{}", max_val);
    println!("{}", idx+1);


}
