// #[macro_export] 注释将宏进行了导出，这样其它的包就可以将该宏引入到当前作用域中，然后才能使用。
// 我们在使用标准库 vec! 时，那是因为 Rust 已经通过 std::prelude 的方式为我们自动引入了

// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
