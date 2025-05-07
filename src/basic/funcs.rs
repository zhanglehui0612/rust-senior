use std::fmt::Debug;

// 接收数组作为参数, 参数类型默认是不可变变量，不能直接修改数组数据
pub fn print (arr: [i32;5]) {
    for item in arr {
        // item = 0; 编译报错
        println!("Receive data: {}", item)
    }
}

// 接收可变数组作为参数, 数组可以被修改
pub fn print2 (arr: &mut[i32;5]) {
    for item in arr.iter_mut() {
        *item *= 2; // 解引用 把每个元素乘以 2
    }
}