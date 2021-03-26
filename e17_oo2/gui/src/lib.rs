pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components:Vec<Box<dyn Draw>>,//实现了Draw trait的对象用dyn修饰,不用泛型的原因是 泛型是静态分配，编译的时候就已经确定类型
}

impl Screen{
    pub fn run(&self){
        for comp in &self.components{
            comp.draw();
        }
    }
}

pub struct Button{
    pub width:u32,
    pub height:u32,
    pub label:String,

}
impl Draw for Button{
    fn draw(&self) {
        println!("draw button, width = {},height = {}, label ={}",self.width,self.height,self.label);
    }
}

pub struct SelectBox{
    pub width:u32,
    pub height:u32,
    pub option:Vec<String>,

}
impl Draw for SelectBox{
    fn draw(&self) {
        println!("draw SelectBox, width = {},height = {}, option ={:?}",self.width,self.height,self.option);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
