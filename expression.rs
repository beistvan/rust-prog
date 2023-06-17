fn give_answer() -> String {
  let answer = "say: green".to_string();
  answer
}

fn main() {
    println!("Hello, Rust!");
    println!("{}", give_answer());
}