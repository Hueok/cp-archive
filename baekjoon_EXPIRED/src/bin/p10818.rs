use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let nums: Vec<i32> = it.map(|s| s.parse().unwrap()).collect();

    // let &min: &i32 = nums.iter().min().unwrap();
    // let &max: &i32 = nums.iter().max().unwrap();


    // let mut mn = i32::MAX;
    // let mut mx = i32::MIN;
    // for x in nums{
    //     mn = mn.min(x);
    //     mx = mx.max(x);
    // }

    let (min, max) = nums.iter().fold(
        (i32::MAX, i32::MIN),
        |(mn, mx), &x| {
            (mn.min(x), mx.max(x))
        }
    );


    println!("{} {}", min, max);
}
