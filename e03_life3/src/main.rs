//方法中的生命周期
#[derive(Debug)]
struct StuA<'a>{
    name:&'a str,
}
impl <'b> StuA<'b>{
    fn do_somthing(&self) ->i32{
        3
    }
    fn do_somthing2(&self,s:&str)->&str{
        // 等价于 fn do_somthing2<'b>(&'b self,s:&str)->&'b str{
        self.name
    }

    fn do_somthing3<'a>(&self,s:&'a str)->&'a str{
        s
    }
}
fn main() {
    let s = String::from("hello");
    let a = StuA{name:&s};
    println!("{}",a.do_somthing());
    let s2 = String::from("hello");
    println!("{}",a.do_somthing2(&s2));
    println!("{}",a.do_somthing3(&s2));
}
