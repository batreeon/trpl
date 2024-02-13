fn main() {
    let s = String::from("hello");

    let l = calculate_length(&s);

    println!("the length of '{}' is {}.", s, l);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
