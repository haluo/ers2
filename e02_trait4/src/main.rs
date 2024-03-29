//对任何实现了特定trait的类型有条件的实现另一trait

trait GetName{
    fn get_name(&self)->&String;
}
trait PrintName{
    fn print_name(&self);
}
//对任何实现Getname 的类型 实现 PrintName
impl<T:GetName> PrintName for T{
    fn print_name(&self) {
        println!("name = {}",self.get_name());
    }
}
struct Student{
    name:String,
}
impl GetName for Student {
    fn get_name(&self) ->&String {
        &self.name
    }
}


fn main() {
    let s = Student{name:String::from("xiaoming")};
    s.print_name();
}
