use std::io::{self, Read};

// math
// assume a = bq + r, then gcd(a,b) = gcd(b,r)
// gcd(a,b) = gcd(b, a mod b)
// lcm(a,b) = a*b / gcd(a,b)

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    let _gcd = gcd(a,b);
    let _lcm = a*b / _gcd;
    println!("{}", _gcd);
    println!("{}", _lcm);

}

fn gcd(x: i32, y:i32) -> i32 {
    if x % y == 0{
        return y;
    }
    gcd(y, x % y)
}
