#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32,height: u32)-> Rectangle{
        Rectangle{width:width,height:height}
    }

    // pub fn area(&self) -> u32 {
    //     self.width * self.height
    // }
     pub fn area(rectangle:&Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

// fn main() {
//     let rect1 = Rectangle::new(30,50 );

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }