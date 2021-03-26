use gui::{Screen,Button,SelectBox};

fn main() {
    let s = Screen{
        components:vec![
            Box::new(Button{
                width:10,
                height:20,
                label:String::from("ok"),
            }),
            Box::new(SelectBox{
                width:20,
                height:30,
                option:vec![String::from("yes"),String::from("no")]
            })
        ]
    };
    s.run();
    println!("Hello, world!");
}
