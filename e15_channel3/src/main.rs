use std::thread;
use std::sync::mpsc;
use std::time::Duration;
//多个发送者
fn main() {
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move||{
        let vars = vec![String::from("hi"),String::from("from"),String::from("thread1")];
        for var in vars{
            tx1.send(var).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move||{
        let vars = vec![String::from("i"),String::from("am"),String::from("thread2")];
        for var in vars{
            tx2.send(var).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for re in rx{
        println!("got : {}",re);
    }
}
