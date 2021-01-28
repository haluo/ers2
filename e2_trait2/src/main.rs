//1.trait bound语法
//2.指定多个trait bound,
//3.返回trait的类型

//直接作为参数的写法
// fn print_information(item:impl GetInformation){
//     println!("print_information:  name={},age={}",item.get_name(),item.get_age());
// }
//使用trait bound 的 写法
// fn print_information<T:GetInformation>(item:T){
//     println!("print_information:  name={},age={}",item.get_name(),item.get_age());
// }

fn main() {
    println!("Hello, world!");
}
