pub fn string_out() {
    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);
    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);
    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);
    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);
    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";println!("{}", byte_escape);
    //一般与c语言使用的是CString
}