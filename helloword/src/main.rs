mod Vecc;
mod stringg;
mod array;
use crate::Vecc::test;
use crate::stringg::string_out;
use crate::array::array;
fn main() {
    
    // let a: u32 =1;
    // let b =0.1;
    // println!("Hello, world! {}",a + b);
    // test();
    let a = 10;
    //新的地址块
    let mut a = 10;
    let a = 11;
    println!("{}", a);
    string_out();
    array();
    
    
}
