// rust 是强类型语言，变量类型不可改变，不过可以声明新的变量，覆盖之前同名变量的定义
fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // number = 3;
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
