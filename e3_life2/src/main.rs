//1.结构体中的生命周期
#[derive(Debug)]
struct A<'a>{
    name:&'a str,
}

fn main() {
    let n = String::from("hello");
    let a  = A{name:&n};
    println!("a = {:?}",a);
}

//2.生命周期的省略
fn get_a_str(x:&str)->&str{
    x
}
//(1)没有生命周期注解却能够编译
//(2)遵守生命周期省略规则的情况下能明确变量的生命周期，则无需明确指定生命周期。函数（方法）的参数的生命周期称为输入生命周期
//而返回值的生命周期称为输出生命周期
//(3)编译器采用三条规则判断引用何时不需要生命周期注解，当编译器检查完这三条后仍不能计算出引用的生命周期，则会停止并生成错误
//(4)生命周期注解省略规则适用于fn定义以及impl代码块定义，如下：
//   a.每个引用参数都有他自己的生命周期参数，例如 如下
//         一个引用参数的函数，其中有一个生命周期 fn foo<'a>(x:&'a i32)
//         两个引用参数的函数则有2个生命周期   fn foo<'a,'b>(x:&'a i32,y:&'b i32) 
//         以此类推    
//   b、如果只有一个输入生命周期参数，那么他被赋予所有输出生命周期参数：
//         fn foo(x:&i32)->&i32  等价于 fn foo<'a>(x:&'a i32)->&'a i32
//   c，如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为&self或&mut self，那么
//      self的生命周期被赋予所有输出生命周期参数 
//  