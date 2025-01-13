// TODO: Fix the compiler error.
// 不可变变量不能修改
fn main() {
    // let x = 3;
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
