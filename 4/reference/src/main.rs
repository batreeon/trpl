fn main() {
    let s = String::from("hello");
    let l = calculate_length(&s);
    // s的所有权没有转移给calculate_length
    println!("the length of '{}' is {}.", s, l);

    let s = String::from("inmutable");
    // 不可变引用
    change1(&s);

    let mut s = String::from("mutable");
    // 可变引用
    change2(&mut s);

    // 已经创建了一个变量的可变引用，不能再创建这个变量的引用

    // 不能在同一时间多次将 s 作为可变变量借用
    let mut s = String::from("race");
    let r1 = &mut s;
    // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("{}", r1);

    let mut s = String::from("race");
    let r11 = &mut s;
    // 这里r11是可变引用，接着又创建了r22可变引用，这是合法的。因为 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    let r22 = &mut s;
    println!("{}", r22);

    // r1,r2两个不可变引用可以同时存在，但是不能再创建可变引用r3了
    let mut s = String::from("race");
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);

    let mut s = String::from("race");
    let r111 = &s;
    let r222 = &s;
    println!("{}, {}", r111, r222);
    let r333 = &mut s;
    println!("{}", r333);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change1(s: &String) {
    println!("change1 1: {}", s);
    // s.push_str("_change"); // error[E0596]: cannot borrow `*s` as mutable, as it is behind a `&` reference
    println!("change1 2: {}", s);
} 

fn change2(s: &mut String) {
    println!("change2 1: {}", s);
    s.push_str("_change");
    println!("change2 2: {}", s);
} 