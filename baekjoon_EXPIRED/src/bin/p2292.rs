use std::io::{self, Read};

// 1, 6, 12, 18, ... ,6n
// x>1, a(x) is common difference sequence. 6(n-1)
// acc -> where specified target number located!

// S(n) = 1+3n(n-1)
// S(n-1) < T <= S(n)
// solution = n

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = it.next().unwrap().parse().unwrap();

    let mut k = 1;
    while (3*k*(k-1)+1) < n {
        k += 1;
    }
    println!("{}", k);
}


// 3n^2 -3n +1 > n^2
// 2n^2 -3n +1 > 0
// (2n-1)(n-1) > 0
// where n >= 0, f(n)>=0. Then, n can be start from sqrt(input).
