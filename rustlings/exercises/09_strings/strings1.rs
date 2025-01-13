// current_favorite_color函数返回的的是String, 而不是&str

// TODO: Fix the compiler error without changing the function signature.
fn current_favorite_color() -> String {
    String::from("blue")
}

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {answer}");
}
