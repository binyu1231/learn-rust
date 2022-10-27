use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} form the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 等待新线程结束，再执行主线程代码
    // handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    // 等待所有进程结束
    handle.join().unwrap();

    // else 主线程结束，其他线程自动结束

    let v = vec![1, 2, 3];
    // rust 不知道线程会存在多久，所以需要将所有权移入线程的闭包中
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

