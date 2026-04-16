use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    while let Some(s) = it.next() {
        if s == String::from('0') {
            break;
        }
        let is_pal = (0..s.len()/2) 
            .all(|i| s.as_bytes()[i] == s.as_bytes()[s.len()-1-i]);

        println!("{}", if is_pal {"yes"} else {"no"});


    }
}
