
//(1)进程是资源分配的最小单位，线程是cpu调度的最小单位
//(2)在使用多线程时会遇到一些问题
//     1、竞争状态：多个线程以不一致的顺序访问数据或资源
//     2、死锁：两个线程相互等待对方停止使用其所拥有的资源，造成2者都永久等待
//     3、只会发生在特定情况下 且难以稳定重现和修复的bug
//(3) 编程语言提供的线程叫绿色线程，如go语言，在底层实现了M:N的模型。即M个绿色线程对应N个OS线程。但是
//rust标准库只提供1：1的线程模型实现，即一个rust线程对应一个OS线程
//
//    运行时代表二进制文件中包含的由语言本身提供的代码，这些代码根据语言的不同可大可小。不过非汇编语言
//都会有一定数量的运行时代码。通常，大家说一个语言”没有运行时“，是指这个语言的运行时很小，如 rust。c语言 
//都是几乎没有运行时的
use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(||{
        for i in 0..10{
            println!("num in sub thread is {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //handle.join().unwrap();
    for i in 0..5{
        println!("num in main thread is {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
