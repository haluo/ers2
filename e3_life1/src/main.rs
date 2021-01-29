//1.rust 中每一个引用都有生命周期，也就是引用保持有效的作用域，大部分时候生命周期是隐含
//并可以推断的 ，正如大部分时候类型可以推断一样
//2.生命周期主要目标是避免悬垂引用
//3.rust编译器使用借用检查器来检查生命周期是否有效
fn main() {
    //错误的例子
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r = {}",r);

    let s1 = String::from("aaa");
    let s2 = String::from("1");
    let s3 = longest(s1.as_str(), s2.as_str());
    println!("s3 = {}",s3);

}

//函数中的生命周期
// fn longest(x:&str,y:&str)->&str{
fn longest<'a>(x:&'a str,y:&'a str)->&'a str{
    if x.len()>y.len(){
        x
    }else{
        y
    }
}
fn get_str<'a>(x:&'a str,y:&str)->&'a str{
    x
}
//错误的例子
// fn a_str<'a>(x:&'a str,y:&'a str)->&'a str{
//     let r = String::from("a");
//     r.as_str()
// }