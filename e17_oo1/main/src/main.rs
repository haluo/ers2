use getaver;
fn main() {
    let mut a = getaver::AverCollect::new();
    a.add(1);
    println!("aver = {}",a.average());
    a.add(2);
    println!("aver = {}",a.average());
    a.add(3);
    println!("aver = {}",a.average());
    a.remove();
    println!("aver = {}",a.average());

}
