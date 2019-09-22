#[derive(Debug, Clone)]
pub enum TokenType{
    Nothing,    // 空关键词
    Keyword,    // 关键字
    Identify, // 标识符
    Text,       // 文本（字符串）
    Number,     // 数字类型
    Separator,  // 行结束符
    End         // 文件结束符
}

// 单词结构，后续还会加上列数
#[derive(Debug, Clone)]
pub struct Token{
    line_num: i32,   // 位于第几行
    text: String,    // 文字是什么
    token_type: TokenType
}


impl Token {
    const EOF:i32 = -1;  // 定义文件结束符
    const NOF:i32 = 0;   // 定义空token

    pub fn new(line_num: i32, text: String, token_type: TokenType)->Self{
        return Token{
            line_num: line_num,
            text: text,
            token_type: token_type
        }
    }

    // 返回终结token
    pub fn end()->Self{
        return Token::new(Token::EOF, "".to_string(), TokenType::End);
    }
    
    // 返回空 token
    pub fn nothing()->Self{
        return Token::new(Token::NOF, "".to_string(), TokenType::Nothing);
    }

    /**
     * 获取token所在的行数
     */
    pub fn get_line_num(&self)->i32{
        return self.line_num;
    }

    /**
     * 获取是否是终结符token
     */
    pub fn is_end(&self)->bool{
        return match self.token_type {
            TokenType::End => true,
            _ => false
        };
    }

    /**
     * 是分隔符（）
     */
    pub fn is_separator(&self)->bool{
        match self.token_type {
            TokenType::Separator => true,
            _ => false
        }
    }

    // 存在行数，即不是空token，也不是文件结束符
    pub fn has_location(&self)->bool{
        return self.line_num != Token::EOF && self.line_num != Token::NOF;
    }

    // pub fn is_nothing(&self)->bool{
    //     return match self.token_type {
    //         TokenType::Nothing => true,
    //         _ => false
    //     };
    // }

    pub fn get_text(&self)->&str{
        return &self.text;
    }

    pub fn get_type_name(&self)->&str{
        match self.token_type {
            TokenType::Nothing => "空单词",
            TokenType::Keyword => "关键字",
            TokenType::Identify => "标识符",
            TokenType::Number => "整数",
            TokenType::Text => "文本",
            TokenType::Separator => "行结束符",
            TokenType::End => "文件结束",
        }
    }

    /**
     * 是数字类型的token，严格来说，目前是整数类型的token
     */
    pub fn is_number(&self)->bool{
        match self.token_type {
            TokenType::Number => true,
            _ => false
        }
    }

    /**
     * 是标识符token
     */
    pub fn is_identify(&self)->bool{
        match self.token_type {
            TokenType::Identify => true,
            _ => false
        }
    }

    pub fn is_text(&self)->bool{
        match self.token_type {
            TokenType::Text => true,
            _ => false
        }
    }
}