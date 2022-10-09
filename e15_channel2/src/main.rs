use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();
    thread::spawn(move ||{
       let vars = vec![String::from("hi"),String::from("from"),String::from("thread")];
        for v in vars{
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    for re in rx{
        println!("got ：{}",re);
    }
}
