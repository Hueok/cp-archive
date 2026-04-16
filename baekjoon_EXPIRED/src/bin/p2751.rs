use std::io::{self, Read, BufWriter, Write};

fn main() {
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = it.map(|s| s.parse().unwrap()).collect();

    nums.sort_unstable();

    for &e in nums.iter(){
        let _ = writeln!(writer, "{}", e);
    }
}
