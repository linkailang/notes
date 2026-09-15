
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

fn foo(s:String)->String{
    println!("{s}");
    s
    //s 没有表达式，作为返回值
    // 相当于return s;
}
fn main(){
    let s1 = String::from("I am a superman.");
    let s1 = foo(s1);
    println!("{s1}");
}
// s1 所有者 → foo(s1) 所有权转给参数 s → s 返回 → 所有权回到新的 s1


//介绍所有权
//创建 s1
// 栈           堆
//┌──────┐    ┌──────────────────────┐
//│ s1   │───→│ "I am a superman."   │
//│ ptr  │    │ len: 16              │
//│ len  │    │ cap: 16              │
//│ cap  │    └──────────────────────┘
//└──────┘
//所有者: s1

//foo(s1) — 所有权转移给函数参数
//栈           堆
// ┌──────┐    ┌──────────────────────┐
// │ s1   │ ×  │                      │ ← s1 失效，不能再用了
// └──────┘    │ "I am a superman."   │
            // │ len: 16              │
// ┌──────┐    │ cap: 16              │
// │ s    │───→└──────────────────────┘
// └──────┘
// 所有者: s（foo 函数的参数）

//fn foo(s: String) -> String {
    // println!("{s}");  // 使用 s，打印内容
    // s                 // 把 s 的所有权返回给调用者
// }
//给到 foo(s1)的调用者
// 栈           堆
// ┌──────┐    ┌──────────────────────┐
// │ s    │ ×  │                      │ ← s 离开作用域，失效
// └──────┘    │ "I am a superman."   │
            // │ len: 16              │
// ┌──────┐    │ cap: 16              │
// │ s1   │───→└──────────────────────┘
// └──────┘
// 所有者: s1（新的 s1）
// println!("{s1}");

