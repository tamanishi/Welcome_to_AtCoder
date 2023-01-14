use std::io::stdin;

fn main() {
    let scan = stdin();
    let mut line1 = String::new();
    let _ = scan.read_line(&mut line1);
    let mut line2 = String::new();
    let _ = scan.read_line(&mut line2);
    let mut line3 = String::new();
    let _ = scan.read_line(&mut line3);
    let value1 = line1.trim().parse::<i32>().unwrap();
    let value2 = line2
        .trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{} {}", value1 + value2[0] + value2[1], line3.trim());
}
