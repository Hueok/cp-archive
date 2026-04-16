use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _t: usize = it.next().unwrap().parse().unwrap();

    for _ in 0.._t {
        let _r: usize = it.next().unwrap().parse().unwrap();
        let word = String::from(it.next().unwrap());
        let result = make_string(&word, _r);
        println!("{}", result);
    }

}

fn make_string(fraction: &str, r: usize) -> String{
    let dict: Vec<char> = fraction.chars().collect();
    let mut result = String::new();
    for alp in dict {
        result.push_str(&alp.to_string().repeat(r));
    }
    return result;
}
