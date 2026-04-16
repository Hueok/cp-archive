use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let k: usize = it.next().unwrap().parse().unwrap();

    let mut dp: Vec<i32> = vec![0; k+1];
    // 1D DP with overwrap

    for i in 1..=n{
        for j in (0..=k.min(i)).rev(){
            if i==j || j==0{
                dp[j] = 1;
                continue;
            }
            dp[j] = dp[j] + dp[j-1];
        }
    }
    println!("{}", dp[k]);

    
}
