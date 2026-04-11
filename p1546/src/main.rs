use std::io::{self, Read};


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    // specify type conventionally

    let (sum, mx) = it
        .map(|x| x.parse::<i32>().unwrap())
        .fold((0_i64, i32::MIN), |(sum, mx), a| {
            (sum + a as i64, mx.max(a))
        });

    let result: f64 = sum as f64 * 100.0 / mx as f64 / n as f64;

    println!("{:.6}", result);
    // cast type when only it being needed

}
