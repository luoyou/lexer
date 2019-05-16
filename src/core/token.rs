
pub struct Token{
    line_num: i32
}

impl Token{
    const EOF:i8 = -1;
    pub const EOL:&'static str = "\n";

    pub fn new(line_num: i32)->Self{
        Token{
            line_num: line_num
        }
    }
}