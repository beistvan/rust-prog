fn main() {
    let add = |n1, n2| n1 + n2;
    let (from, to) = (3, 10);
    let sum_range = || {
        let mut sum = 0;
        
        for n in from..to {
            sum = add(sum, n);
        }
        
        sum
    };
    
    println!("sum = {}", sum_range()); // sum = 42
}
