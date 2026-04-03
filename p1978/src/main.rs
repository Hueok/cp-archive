use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _N: usize = it.next().unwrap().parse().unwrap();

    let nums: Vec<usize> = it.map(|x| x.parse().unwrap()).collect();
    let sqrts: Vec<_> = nums.iter().map(|&x| (x as f64).sqrt().floor() as usize).collect();

    let mut count: usize = 0;

    for (idx, &val) in nums.iter().enumerate() {
        if val == 1 {
            continue;
        }
        let current = &nums[idx];
        let current_sqrts = &sqrts[idx];
        let mut is_prime : bool = true;

        for j in 2..=*current_sqrts {
            if *current%j == 0{
                is_prime = false;
                break;
            }
        }
        if is_prime {
            count += 1;
        }
    }
    println!("{}", count);

}
