use std::thread;
//需要添加move将v移动到线程内部， 因为不获取所有权的话 线程内部，不知道v能在
//主线程中存活多久，可能被提前drop掉

fn main() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move||{
       println!("v = {:?}",v) ;
    });
    //主线程无法使用V
    handle.join().unwrap();
}
