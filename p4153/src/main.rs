use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    loop {
        let mut squared: Vec<u32> = it.by_ref()
                                .take(3)
                                .map(|s| {
                                    let v: u32 = s.parse().unwrap();
                                    v * v
                                })
                                .collect();

        if squared.iter().sum::<u32>() == 0 {
            break;
        }

        squared.sort();

        let (a,b,c) = (squared[0], squared[1], squared[2]);

        if a+b == c {
            println!("right");
        } else {
            println!("wrong");
        }
    }

}
