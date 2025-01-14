// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // TODO: Fix the compiler error by moving one line.

    // longest函数中的生命周期'a 的大小将等于 x 和 y 中较小的那个
    // 所以longest返回result的生命周期等于string2的生命周期，且string2生命周期在大括号尾部后就结束了
    // 但是，大括号后仍在使用result；

    // Rust语言圣经(Rust Course)中解释：
    // 在 longest 函数中，string2 的生命周期也是 'a，由此说明 string2 也必须活到 println! 处；
    // 可是 string2 在代码中实际上只能活到内部语句块的花括号处 }，小于它应该具备的生命周期 'a，因此编译出错。
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(&string1, &string2);
    // }
    // println!("The longest string is '{result}'");

    // 方式1: 改变string2，使其与string1保持一致
    // let string1 = String::from("long string is long");
    // let string2 = String::from("xyz");
    // let result;
    // {
    //     result = longest(&string1, &string2);
    // }
    // println!("The longest string is '{result}'");

    // 方式2: 在result的生命周期内去使用result
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        println!("The longest string is '{result}'");
    }
}
