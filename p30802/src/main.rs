use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _N: usize = it.next().unwrap().parse().unwrap();
    let order: Vec<usize> = it.by_ref().take(6)
                            .map(|v| v.parse().unwrap())
                            .collect();

    let _T: usize = it.next().unwrap().parse().unwrap();
    let _P: usize = it.next().unwrap().parse().unwrap();

    let mut total_T = 0;
    for e in order {
        let cnt = (e + _T -1) / _T;
        total_T += cnt;
    }
    
    let p_q = _N / _P;
    let p_r = _N % _P;

    println!("{}", total_T);
    println!("{} {}", p_q, p_r);
}
