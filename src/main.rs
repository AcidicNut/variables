use std::io;

fn main() {
    let mut n = String::new();
    println!("num here");

    io::stdin().read_line(&mut n).expect("Ohh no");

    let n: u32 = n.trim().parse().expect("nut num");

    let mut first = 0;
    let mut second = 1;
    let mut i = 1;

    let result = loop {
        if i == n {
            break second;
        }
        i += 1;
        let temp = first;
        first = second;
        second += temp;
    };

    println!("result: {}", result);
}
