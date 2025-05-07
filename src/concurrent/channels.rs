use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

pub fn transport() {
    let (sender, receiver) = mpsc::channel();
    // 子线程发送数据
    for i in 0..=2 {
        let clone = sender.clone(); // 克隆发送者
        thread::spawn(move || {
            let data = format!("{} {}", "hello-", i);
            clone.send(data).unwrap();
        });
    }
    drop(sender); // 主线程显式关闭发送者，表明没有更多消息

    // 主线程接收数据
    // 只接受一条消息
    // let data = receiver.recv().unwrap();
    for data in receiver {
        println!("主线程接收数据: {data}");
    }
}


// 多个线程（生产者）发送消息到主线程（消费者）
pub fn multi_sender() {
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // 开启5个子线程,向通道发送数据
    for i in 0..5 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            let msg = format!("Message from thread {}", i);
            tx_clone.send(msg).unwrap();
        });
        handles.push(handle);
    }

    // 等待所有子线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }

    // 主线程接收数据
    for received in rx.try_iter() {
        println!("Received: {}", received);
    }
}


// 一个线程生成数据并发送到队列，多个线程消费队列中的数据
pub fn send() {
    let (tx, rx) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(rx));

    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 1..=10 {
            println!("Produced: {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 消费者线程
    let mut consumers = vec![];
    for _ in 0..3 {
        let rx_clone = Arc::clone(&receiver);
        let consumer = thread::spawn(move || {
            while let Ok(data) = rx_clone.lock().unwrap().recv() {
                println!("Consumed: {}", data);
            }
            // 和上面是等价的
            // loop {
            //     let received = rx_clone.lock().unwrap().recv();
            //     match received {
            //         Ok(data) => println!("Consumed by thread {:?}: {}", thread::current().id(), data),
            //         Err(_) => break, // 通道关闭，退出循环
            //     }
            // }
        });
        consumers.push(consumer);
    }

    producer.join().unwrap();
    for consumer in consumers {
        consumer.join().unwrap();
    }
}