// enum TokenType{
//     Keyword(String),    // 关键字
//     Symbol(String),     // 字符
//     Identidify(String), // 标识符
//     Number(i32)         // 数字类型 
// }

pub trait Token{
    fn new(line_num: u32, text: String)->Self;

    fn get_line_num(&self)->u32;

    fn is_identifier(&self)->bool{
        return false;
    }

    fn is_number(&self)->bool{
        return false;
    }

    fn is_string(&self)->bool{
        return false;
    }

    fn get_number(&self)->i32{
        panic!("该类型非整数类型");
    }

    fn get_text(&self)->&str{
        return "";
    }

}

// 单词结构，后续还会加上列数
pub struct Word{
    line_num: u32,  // 位于第几行
    text: String    // 文字是什么
}

impl Word{
    pub const EOF:u32 = 0;  // 定义文件结束符
}

pub struct IdToken{
    word: Word
}

impl Token for IdToken{
    
    fn new(line_num: u32, text: String)->Self{
        let word = Word{
            line_num: line_num,
            text: text
        };
        return IdToken{
            word: word
        }
    }

    fn get_line_num(&self)->u32{
        return self.word.line_num;
    }

    fn is_identifier(&self)->bool{
        return true;
    }

    fn get_text(&self)->&str{
        return &self.word.text;
    }
}

pub struct NumToken{
    word: Word
}

impl Token for NumToken {
    fn new(line_num: u32, text: String)->Self{
        let word = Word{
            line_num: line_num,
            text: text
        };
        return NumToken{
            word: word
        }
    }

    fn get_line_num(&self)->u32{
        return self.word.line_num;
    }

    fn is_identifier(&self)->bool{
        return true;
    }

    fn get_text(&self)->&str{
        return &self.word.text;
    }
}