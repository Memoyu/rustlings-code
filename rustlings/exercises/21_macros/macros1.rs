// 宏调用方式，方式：宏名称!()

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // TODO: Fix the macro call.
    my_macro!();
}
