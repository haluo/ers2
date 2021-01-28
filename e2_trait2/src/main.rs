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


trait GetName{
    fn get_name(&self)->&String;
}
trait GetAge{
    fn get_age(&self)->u32;
}

//写法1
// fn print_information<T:GetName+GetAge>(item:T){
//     println!("print_information:  name={}, age={}",item.get_name(),item.get_age());
// }
//写法2
fn print_information<T:GetName+GetAge>(item:T) where T:GetName+GetAge{
    println!("print_information:  name={}, age={}",item.get_name(),item.get_age());
}

#[derive(Debug)]
pub struct Student{
    pub name:String,
    pub age:u32,
}
impl GetName for Student {
    fn get_name(&self) ->&String {
        &self.name
    }
}
impl GetAge for Student{
    fn get_age(&self) ->u32 {
        self.age
    }
}

//trait 作为返回类型
fn produce_item_with_age()-> impl GetAge{
    Student{name:"xiaoming".to_string(),age:12}
}



fn main() {
    let s = Student{name:"zf".to_string(),age:18};
    print_information(s);

    let s = produce_item_with_age();
    println!("s.age = {}",s.get_age());
}
