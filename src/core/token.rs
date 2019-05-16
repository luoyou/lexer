
pub struct Token{
    line_num: i32
}

impl Token{
    const EOF:i8 = -1;  // 定义文件结束符
    pub const EOL:&'static str = "\n";  // 定义换行符

    pub fn new(line_num: i32)->Self{
        Token{
            line_num: line_num
        }
    }
}