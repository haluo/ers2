//1.trait 用于定义与其他类型共享的功能，类似于其他语言的接口
//（1).可以通过trait以抽象的方式定义共享的行为
//(2).可以通过trait bounds 指定泛型是任何有特定行为的类型
//2.定义trait
pub trait GetInformation {
    fn get_name(&self)->&String;
    fn get_age(&self)->u32;
}
//3.实现trait
pub struct Student{
    pub name:String,
    pub age:u32,
}
impl GetInformation for Student {
    fn get_name(&self)->&String{
        &self.name
    }
    fn get_age(&self)->u32{
        self.age
    }
}
//4.默认实现，可以在定义trait的时候提供默认的行为，trait的类型可以使用默认的行为
pub trait SchoolName {
    fn get_school_name(&self)->String{
        String::from("123")
    }
}
impl SchoolName for Student{}

//5.trait作为参数
fn print_information(item:impl GetInformation){
    println!("print_information:  name={},age={}",item.get_name(),item.get_age());
}

fn main() {
    let s = Student{name:String::from("xiaoming"),age:10};
    println!("s.name = {},s.age = {}",s.get_name(),s.get_age());
    let school = s.get_school_name();
    println!("school = {}",school);
    print_information(s);


}
