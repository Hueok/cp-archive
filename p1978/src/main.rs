use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _N: usize = it.next().unwrap().parse().unwrap();

    let nums: Vec<usize> = it.map(|x| x.parse().unwrap()).collect();

    let count = nums.iter()
        .filter(|&&n| {
            if n == 1 {
                return false;
            }
            let limit = (n as f64).sqrt() as usize;
            (2..=limit).all(|i| n%i != 0)
        })
        .count();

    println!("{}", count);

}
