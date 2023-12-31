fn add (n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let values = [8, 30, 1, 3];
    let mut sum = 0;
    
    for n in &values[0..2] {
        sum = add(sum, *n);
    }
    
    for n in &values[2..4] {
        sum = add(sum, *n);
    }
    
    println!("sum = {}", sum);
}
