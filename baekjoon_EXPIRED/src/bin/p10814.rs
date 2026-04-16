use std::io::{self, BufReader, BufRead, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = BufWriter::new(io::stdout().lock());

    let mut n = String::new();
    reader.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    let mut input = String::new();

    let mut users: Vec<(i32, String)> = Vec::new();

    for _ in 0..n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut it = input.split_whitespace();
        
        let age: i32 = it.next().unwrap().parse().unwrap();
        let name: String = it.next().unwrap().to_string();
        users.push((age, name));
    }

    users.sort_by_key(|p| p.0);

    let bin = (0..n).map(|i| writeln!(writer, "{} {}", users[i].0, users[i].1)).count();

}
