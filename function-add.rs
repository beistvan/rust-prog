fn add (n1: i32, n2: i32) -> i32 {
    return n1 + n2; // return (n1 + n2); // parenthesis give warnings 
}

fn main() {
    let a = 38;
    let b = 4;
    let sum = add(a, b);
    
    println!("{} + {} = {}", a, b, sum);
}

// compile:
// rustc main.rs
// run:
// ./main
