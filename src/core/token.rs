
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

    fn get_text(&self)->String{
        return String::from("");
    }

}

pub struct Word{
    line_num: u32,
    text: String
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
        }
        return IdToken{
            word: Word
        }
    }

    fn get_line_num(&self)->u32{
        return self.word.line_num;
    }

    fn is_identifier(&self)->bool{
        return true;
    }

    fn get_text(&self)->String{
        return self.word.text;
    }
}

// pub struct NumToken{
//     word: Word
// }

// impl Token for NumToken{
//     pub fn is_number(&self)->bool{
//         return true;
//     }

//     pub fn get_text(&self)->String{
//         return self.word.text;
//     }

//     pub fn get_number(&self)->String{
//         return self.word.text;
//     }
// }