fn main() {
    let mut s = String::from("hello");
    let z = s.clone();
    s.push_str(", world!");

    let x = 4;
    let y = x;
    let x = 32;
    println!("{}", s);
    println!("{}", z);
    println!("{}", x);
    println!("{}", y);
}
