mod rectangle;
use crate::rectangle::Rectangle;

fn main() {
    let rect1 = Rectangle::new(30,50);
    println!(
        "The area of the rectangle is {} square pixels.",
        // rect1.area()
        Rectangle::area(&rect1)
    );
}

// struct Circle{
//     //定义结构体计算圆周长
//     private x:f64,  //8字节
//     y:f64,
//     radius:f64,
// }
// //声明结构体内部属性
// //impl 关键字定义方法
// impl Circle{
//     /// new 结构体的实例。必须要，rust没有魔法（默认new不类似java）
//     fn new(x:f64,y:f64,radius:f64)->Circle{
//         Circle{x,y,radius}
//         //字面量（literal）就是代码里直接写出来的、固定不变的值——你写它是什么，它就是什么
//     }

//     fn circumference(&self) -> f64 {
//         2.0 * std::f64::consts::PI * self.radius
//     }
    
// }



// fn main() {
//     let c = Circle::new(0.0, 0.0, 5.0);  // 关联函数调用
//     println!("周长: {}", c.circumference()); // 方法调用
// }

// fn new(x:f64,y:f64,radius:f64,)->Circle{
// Circle 返回值类型 result

//self 表示调用这个方法的结构体实例本身（类似 Python 的 self、Java/C++ 的 this）
//& 表示借用（不可变引用）——方法只是读取实例的数据，不修改它，也不拿走所有权
//正因为有 self 参数，它才叫方法（可以用 . 调用，比如 c.circumference()）；没有 self 的叫关联函数（用 :: 调用，比如 Circle::new(...)）


