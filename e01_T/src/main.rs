//函数使用T
//PartialOrd 表示可排序 Copy 表示具有Copy 特征
fn largest<T:PartialOrd+Copy>(list:&[T]) ->T{
    let mut larger = list[0];
    for &item in list.iter(){
        if item > larger{
            larger = item;
        }
    }
    larger
}
//结构体使用T
#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}
#[derive(Debug)]
struct Point2<T,U>{
    x:T,
    y:U,
}
//枚举中使用T
enum Option<T>{
    Some(T),
    None
}
enum Result<T,E>{
    Ok(T),
    Err(E),
}

//struct方法内使用T
impl<T> Point<T>{
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T{
        &self.y
    }
}
impl<T,U> Point2<T,U>{
    fn create<V,W>(self,other:Point2<V,W>) -> Point2<T,W>{
        Point2{
            x:self.x,
            y:other.y,
        }
    }
}

fn main() {
    let num_list = vec![1,2,3,12];
    let largest_num = largest(&num_list);
    println!("largest_num = {}",largest_num);
    let char_list = vec!['a','b','d','c'];
    let largest_char = largest(&char_list);
    println!("largest_char = {}",largest_char);
    let num_list2 = [1,2,3,111];
    let largest_num = largest(&num_list2);
    println!("largest_num = {}",largest_num);
    println!("-------------------------------");
    let int_point = Point{x:1,y:2};
    println!("int_point = {:?}",int_point);
    let point2 = Point2{x:1,y:'a'};
    println!("point2 = {:?}",point2);
    println!("-------------------------------");
    let x = int_point.get_x();
    println!("x = {}",x);

    let p1 = Point2{x:5,y:1.1};
    let p2 = Point2{x:"hello",y:'c'};
    let p3 = p1.create(p2);
    println!("p3 = {:?}",p3);
}
