struct Number {
    odd: bool,
    num: i32,
}
fn main() {
    

    let n1 = Number { odd: true, num: 32};
    let n2 = Number { odd: false, num: 31};

    print_number(n1);
    print_number(n2);
}

fn print_number(n: Number) {
    if( n.odd) {
        println!("Odd Number {}", n.num);
    } else {
        println!("Even Number {}", n.num);
    }
    
}
