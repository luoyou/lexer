pub enum TokenType{
    Keyword,    // 关键字
    Identidify, // 标识符
    Number,     // 数字类型
    Comment,    // 注释
    End         // 文件结束符
}

// 单词结构，后续还会加上列数
pub struct Token{
    line_num: u32,  // 位于第几行
    text: String,    // 文字是什么
    token_type: TokenType
}


impl Token {
    pub const EOF:u32 = 0;  // 定义文件结束符

    pub fn new(line_num: u32, text: String, token_type: TokenType)->Self{
        return Token{
            line_num: line_num,
            text: text,
            token_type: token_type
        }
    }

    pub fn get_line_num(&self)->u32{
        return self.line_num;
    }

    pub fn get_type(&self)->&TokenType{
        return &self.token_type;
    }

    pub fn is_end(&self)->bool{
        return match self.token_type {
            TokenType::End => true,
            _ => false
        };
    }

    pub fn get_text(&self)->&str{
        return &self.text;
    }

    pub fn get_type_name(&self)->&str{
        match self.token_type {
            TokenType::Keyword => "关键字",
            TokenType::Identidify => "标识符",
            TokenType::Number => "整数",
            TokenType::Comment => "注释",
            TokenType::End => "文件结束",
        }
    }
}