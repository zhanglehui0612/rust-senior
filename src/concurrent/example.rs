// use std::{thread, io};
// use std::sync::Arc;
// use std::thread::spawn;
//
// // 用于函数返回值时，() 表示函数完成任务，但不返回实际的数据
// pub fn process_files_in_parallel(filenames: Vec<String>) -> io::Result<()> {
//     // 定义线程数常量
//     const NTHREADS: usize = 8;
//
//     // 将任务根据指定的线程数量划分成多个chunks
//     let worklists = split_chunks(filenames, NTHREADS);
//
//     // 保存线程句柄列表
//     let mut handles = vec![];
//
//     // 为每个任务列表启动一个线程
//     for worklist in worklists {
//         // 创建线程，处理对应的任务列表
//         // `move` 关键字将捕获的变量 `worklist` 的所有权转移到线程中
//         // worklist由父线程循环定义
//         // move闭包将worklist移入到闭包中
//         // spawn将闭包移到新的子线程
//         handles.push(thread::spawn(move || process_files(worklist)));
//     }
//
//     // Main线程等待所有线程完成任务
//     for handle in handles {
//         // 调用 `join` 方法等待8个线程完成
//         // `join` 返回一个 `Result`，表示线程的执行状态
//         // 使用 `unwrap` 确保线程没有 panic
//         // 使用 `?` 传播 `process_files` 中可能的 I/O 错误
//         handle.join().unwrap()?
//     }
//     // Ok(()) 表示一个成功的 Result，其中成功的值是 ()（unit 类型）
//     // 函数成功返回，`Ok(())` 表示一个成功的 `io::Result`
//     Ok(())
// }
//
// // 每一个线程开始处理具体的任务列表，根据处理结果返回
// fn process_files(filenames: Vec<String>) -> io::Result<()> {
//     for filename in filenames {
//         println!("Processing file: {}", filename);
//         // 假设处理文件的逻辑...
//     }
//     Ok(())
// }
//
// // 根据线程数量划分多个认为列表，每一个线程负责一部分任务列表
// fn split_chunks<T>(items: Vec<T>, n: usize) -> Vec<Vec<T>> {
//     // items.chunks根据数量划分chunk
//     // 对chunk进行映射，转化成一个vector列表
//     // 将所有chunk转化后的列表收集起来，构成Vec<Vec<T>>即worklists
//     // items.chunks((items.len() + n - 1) / n).map(|chunk| chunk.to_vec()).collect()
// }
//
//
// struct GigabyteDict{}
// //
// pub fn process_files_in_parallel_with_arc(filenames: Vec<String>, glossary: Arc<GigabyteDict>) -> io::Result<()> {
//     const NTHREADS: usize = 8;
//     let worklists = split_chunks(filenames, NTHREADS);
//     let mut handles = vec![];
//
//
//     for worklist in worklists {
//         // 为每一个线程对Arc智能指针包裹的GigabyteDict对象创建一个共享所有权对象，在堆上创建GigabyteDict，每一个Arc指针都会指向这个堆上的GigabyteDict数据，每次clone一个，引用计数器原子+1
//         // 这样既可以共享数据，又不需要为每一个线程复制一个大对象GigabyteDict传递给线程
//         let glossary_clone = glossary.clone();
//         handles.push(thread::spawn(move || process_files_arc(worklist, &glossary_clone)));
//     }
//
//     for handle in handles {
//         handle.join().unwrap()?
//     }
//
//     Ok(())
// }
//
// // 每一个线程开始处理具体的任务列表，根据处理结果返回
// fn process_files_arc(filenames: Vec<String>, glossary: &Arc<GigabyteDict>) -> io::Result<()> {
//     for filename in filenames {
//         println!("Processing file: {}", filename);
//         // 假设处理文件的逻辑...
//     }
//     Ok(())
// }
//
//
