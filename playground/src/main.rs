fn main() {
    println!("hi");
    println!("Hello, world!");
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("{}",b1);
}
