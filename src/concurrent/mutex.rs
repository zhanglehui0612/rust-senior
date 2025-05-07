// Mutex 确保在任意时刻只有一个线程可以访问或修改受保护的数据，从而避免数据竞争
// 问题拆解
// 线程需要访问数据时，需要转移变量的所有权。
//
// 是的，Rust 的所有权机制要求在线程中使用数据时，要么转移所有权，要么通过引用访问。
// 如果数据通过所有权转移到一个线程中，那么其他线程就无法再访问它。
// 线程结束后数据还需要使用怎么办？
//
// 如果线程拥有数据的所有权，但我们还需要在主线程或其他线程中使用这些数据，Rust 的所有权规则会让我们无法再轻松地使用这些数据
// Arc 智能指针通过对共享变量进行引用计数，当有n个线程的时候，可以clone n个所有者，这些所有者共享所有权, 这也就是Arc+Mutex经常结合使用的原因

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// 线程共享计数器
pub fn share_counter() {
    // 创建一个线程安全的可以共享所有权的计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for e in 0..10 {
        let clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_num = counter.lock().unwrap();
    println!("Final counter value: {}", *final_num);
}

// 生产者和消费者
pub fn queue() {
    // 创建一个队列
    let queue = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    // 生产者线程
    {
        let clone1 = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for i in 1..=10 {
                let mut mq = clone1.lock().unwrap();
                mq.push(i);
                println!("Produced: {}", i);
                drop(mq); // 手动释放锁
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // 消费者线程
    for _ in 0..=3 {
        // 避免消费者并发消费出现问题，同一时刻只允许一个消费者消费
        let clone2 = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            loop {
                let mut mq = clone2.lock().unwrap();
                // if Some(msg)的写法。if的条件需要是一个布尔表达式，而Some(msg)本身并不是一个布尔值，这会导致编译错误
                // if let 是一种语法糖，用于匹配特定的模式，同时可以解构其中的值。以下是 if let 的核心逻辑：
                // if let Some(msg) = mq.pop() 尝试匹配 mq.pop() 的返回值是否为 Some(msg)。
                // 如果成功匹配（即 mq.pop() 返回 Some(value)），那么 msg 会绑定到 value。
                // 如果匹配失败（即返回 None），if let 的代码块会跳过。
                if let Some(msg) = mq.pop() {
                    println!("Consumed: {}", msg);
                } else {
                    break; // 队列为空时退出循环
                }
                drop(mq); // 手动释放锁
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}


struct BankAccount {
    balance: i32,
}
impl BankAccount {
    fn new(balance: i32) -> Self {
        BankAccount { balance }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            return true
        }
        return false
    }
}

// 并发转账
pub fn transfer() {
    let account1 = Arc::new(Mutex::new(BankAccount::new(100)));
    let account2 = Arc::new(Mutex::new(BankAccount::new(200)));

    let mut handles = vec![];

    for _ in 0..10 {
        let acc1 = Arc::clone(&account1);
        let acc2 = Arc::clone(&account2);

        let handle = thread::spawn(move || {
            let amount = 10;
            let mut from = acc1.lock().unwrap();
            let mut to = acc2.lock().unwrap();

            if from.withdraw(amount) {
                to.deposit(amount);
                println!("Transferred {} from Account 1 to Account 2", amount);
            } else {
                println!("Failed to transfer {} from Account 1", amount);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Account 1 Balance: {}", account1.lock().unwrap().balance);
    println!("Final Account 2 Balance: {}", account2.lock().unwrap().balance);
}