fn labs() {
    // 整数, 没有显式声明数据类型就是i32
    let result: u32 = 10 + 20;
    let r = 17;
    // 浮点数, 没有显式声明数据类型就是f64
    let num = 18.8;
    // 布尔值
    let flag: bool = true;
    // 字符类型
    let c = 'z';
    let heart_eyed_cat = '😻';
    // tuple元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 数组
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    // 声明数据类型和长度5
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 声明长度为5，数字3填满整个数组
    let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
}

pub fn foreach() {
    let mut a = 42;

    let b = &mut a;      // 第一次可变借用
    let c = &mut *b;     // 再次可变借用（对 b 指向的值 a）

    println!("{b:?}");   // ❌ 错误：此时 b 被冻结了，不能访问
}