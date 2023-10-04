use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse().unwrap();

    let mut s = [0, 1];
    for i in 2..=n {
        s[i % 2] += s[(i - 1) % 2];
    }

    println!("{}", s[n % 2]);
}
