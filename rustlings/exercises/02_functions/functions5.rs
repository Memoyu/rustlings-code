// 函数指明了返回类型为i32，但是现在返回的为()
// 最后一段不使用;则为

// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // num * num;
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
