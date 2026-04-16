use std::io::{self, Read};

const P: i64 = 1234567891;
const R: i64 = 31;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _l: usize = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap();

    let (_, result) = s.chars().fold((1_i64, 0_i64), |(pow, acc), c| {
        let val = (ctoi(c) * pow) % P;
        let new_pow = (pow * R) % P;
        let new_acc = (acc + val) % P;
        (new_pow, new_acc)
    });

    println!("{}", result);

}

fn ctoi(c: char) -> i64 {
    (c as u8 - b'a') as i64 + 1
}

