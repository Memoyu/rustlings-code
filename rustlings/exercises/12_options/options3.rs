#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: Fix the compiler error by adding something to this match statement.
    // match optional_point {
    //     Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
    //     _ => panic!("No match!"),
    // }

    // 方式1：使用shadowing来获取match 返回的值
    // let optional_point = match optional_point {
    //     Some(p) => {
    //         println!("Co-ordinates are {},{}", p.x, p.y);
    //         p
    //     }
    //     _ => panic!("No match!"),
    // };

    // 方式2：解决值移动问题，改为借用
    match &optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    // 值发生移动导致无法在使用
    println!("{optional_point:?}"); // Don't change this line.
}
