#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect1);
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (
    //  in Nightly builds, run with -Z macro-backtrace for more info)

    println!("rect1 is {:?}", rect1);
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    /*
    = help: the trait `Debug` is not implemented for `Rectangle`
    = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (
        in Nightly builds, run with -Z macro-backtrace for more info)
     */
}
