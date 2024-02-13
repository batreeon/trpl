fn main() {
    let x = 5;
    let y = x; // copy
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("ownership");
    println!("s1 = {}", s1);
    let s2 = s1; // s1 moved
    // println!("s1 = {}, s2 = {}", s1, s2); // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {}", s2);

    let s1 = String::from("ownership");
    let s2 = s1.clone(); // clone
    println!("s1 = {}, s2 = {}", s1, s2);

    let s1 = String::from("owenership");
    println!("s1 = {}", s1);
    ownership_move(s1);
    // println!("s1 = {}", s1); // s1 moved

    let i1 = 32;
    println!("i1 = {}", i1);
    ownership_copy(i1);
    println!("i1 = {}", i1);
}

fn ownership_move(s: String) {
    println!("s = {}", s);
}

fn ownership_copy(i: i32) {
    println!("i = {}", i)
}
