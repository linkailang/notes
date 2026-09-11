
///rust原生支持UTF-8编码字符串，字符串内可以是各国语言。
/// println! ! 跟在标识符后面只有一个含义：这是一个宏调用
/// 
pub fn greet_words(){
    let southern_germany = "Grüß Gott!";
    let cn = "你好,世界";
    let en = "World, hello";
    let lening = [southern_germany,cn,en];
    for greeting in lening.iter(){
        println!("{}",greeting);
    }
    for greeting in &lening{
        println!("{}",greeting);
    }
    
}
// for region in regions { ... }      // 等价于 regions.into_iter() → 消费集合，获取所有权
// for region in &regions { ... }     // 等价于 regions.iter()       → 借用，得到 &T
// for region in &mut regions { ... } // 等价于 regions.iter_mut()   → 可变借用，得到 &mut T

 