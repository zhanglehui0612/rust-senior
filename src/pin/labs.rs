use std::fmt::Debug;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;

#[derive(Debug)]
struct Data {
    value: i32,
}

pub fn example() {
    // 当数据存储在栈上，所有权转移会将整个值复制到新变量的栈位置，而不是简单地移动指针。这种行为可能导致地址变化
    let a = Data { value: 42 }; // 栈上分配
    println!("Address of a: {:p}", &a);

    let b = a; // 所有权转移
    println!("Address of b: {:p}", &b);

    // 当数据存储在堆上（如通过 Box），所有权转移时，仅指针的所有权发生变化，堆上的数据地址不变
    let x = Box::new(42);
    println!("Address of x on stack: {:p}", &x); // 栈上变量 x 的地址
    println!("Address of x on heap: {:p}", x.as_ref()); // 堆上数据地址

    // 转移所有权
    let y = x;
    println!("Address of y on stack: {:p}", &y); // 栈上变量 y 的地址
    println!("Address of y on heap: {:p}", y.as_ref()); // 堆上数据地址

    // let pinned_x = Pin::new(x);
    // println!("Pinned value: {:p}", &pinned_x);


}


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn print_address(self: Pin<&Self>) {
        // 打印 `Point` 和字段的地址
        println!("Pinned Point address: {:p}", self);
        println!("Pinned Point.x address: {:p}", &self.x);
        println!("Pinned Point.y address: {:p}", &self.y);
    }
}

pub fn stack_allocate() {
    let mut p1 = Point { x: 10, y: 20 };
    println!("Original Point address: {:p}", &p1);

    // 将 p1 通过 Pin 固定
    let pinned_p1 = unsafe { Pin::new_unchecked(&mut p1) };
    pinned_p1.as_ref().print_address();

    // 尝试转移所有权（通过 Pin 阻止移动）
    let p2 = p1; // 编译错误，p1 被 Pin 后无法安全地移动
}



pub struct InlineBuffer {
    data: [u8; 64],
    slice: NonNull<[u8]>,
    // _pinned: PhantomPinned,
}

impl InlineBuffer {
    pub fn new() -> Self {
        Self{
            data: [0; 64],
            slice: NonNull::from(&[])
        }
    }

    pub fn set_contents(&mut self, buf: &[u8]) -> bool {
        let buf_len: usize = buf.len();
        if (buf_len > self.data.len()) {
            return false;
        }

        self.data[0..buf_len].copy_from_slice(buf);
        self.slice = NonNull::from(&self.data[0..buf_len]);
        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            &*self.slice.as_ptr()
        }
    }
}

impl Default for InlineBuffer {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}