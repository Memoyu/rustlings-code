// 调用的函数需要传入一个参数，调用时并没有传入

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    // call_me();
    call_me(3);
}
