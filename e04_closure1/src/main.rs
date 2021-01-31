//1.闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数，闭包与函数不同的是
//闭包允许 捕获调用者作用域中的值
//2.闭包的使用方式
//3.使用带有泛型和fn trait的闭包
fn main() {
    let use_closure = ||{
        println!("this is a closure");        
    };
    use_closure();
    //闭包定义会为每个参数和返回值推导一个具体的类型但是不能推导2次不同类型
    let add_one_v2 = |x:u32|->u32{x+1};
    let add_one_v3 = |x|{x+1};
    let add_one_v4 =  |x|x+1;

    let a  = add_one_v1(1);//function
    let b  = add_one_v2(2);//closure
    let c  = add_one_v3(3);//closure
    let d  = add_one_v4(4);//closure

    println!("a = {},b = {}, c = {}, d = {}",a,b,c,d);

    //不能推导2次不同类型的例子
    let example_closure = |x|x;
    let s = example_closure(String::from("hello"));//第一次推导是字符串
    println!("{}",s);
    // let n = example_closure(5);//第二次推导是数字 会报错
    // println!("{}",n);


    //捕获作用域中的变量i
    let i = 1;
    let exe = |x|{x+i};
    let r = exe(5);
    println!("r = {}",r);

}



//语法格式
//函数
fn add_one_v1(x:u32)->u32{
    x+1
}
//闭包 有自动推导类型的能力！！！！
// let add_one_v2 = |x:u32|->u32{x+1};
// let add_one_v3 = |x|{x+1};
// let add_one_v4 =  |x|x+1;