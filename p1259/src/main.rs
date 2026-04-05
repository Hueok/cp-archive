use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    while let Some(s) = it.next() {
        if s == String::from('0') {
            break;
        }
        let size: usize = s.len();
        let numstring: i32 = s.parse().unwrap();
        let mut is_rom: bool = true;

        for i in 1..=size/2{
            let divisor = 10_i32.pow((size-i) as u32);
            let lhs: i32 = (numstring / divisor) % 10;
            let rhs: i32 = (numstring % 10_i32.pow(i as u32)) / 10_i32.pow((i-1) as u32);
            if lhs != rhs {
                is_rom = false;
            }
        }
        
        let mut result = String::new();
        if is_rom {
            result.push_str("yes");
        } else {
            result.push_str("no");
        }
        println!("{}", result);


    }
}
