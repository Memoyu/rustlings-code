// 将vec0所有权转给fill_vec方法，在fill_vec处理完成后，再返回

// TODO: Fix the compiler error in this function.
// 方式1：更改参数，增加可变
// fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // 方式2：获取vec所有权
    // let vec = vec;
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
