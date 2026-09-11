pub fn greet_words(){
    let southern_germany = "Grüß Gott!";
    let cn = "你好,世界";
    let en = "World, hello";
    let lening = [southern_germany,cn,en];
    for len in lening.iter(){
        println!("{}",&len);
    }
}