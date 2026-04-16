use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _N: usize = it.next().unwrap().parse().unwrap();
    let _M: usize = it.next().unwrap().parse().unwrap();

    let nums: Vec<i32> = it.map(|s| s.parse().unwrap()).collect();
    let mut best = 0;
    dfs(&nums, 0, 0, 0, _M, &mut best);
    println!("{}", best);
}

fn dfs(
    nums: &Vec<i32>,
    start: usize,
    depth: usize, 
    sum: i32,
    m: usize,
    best: &mut i32
) {
    if depth == 3{
        if sum <= m as i32{
            *best = (*best).max(sum);
        }
        return;
    }
    for i in start..nums.len(){
        dfs(nums, i+1, depth+1, sum+nums[i], m, best);
    }
}
