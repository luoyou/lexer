use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::token::Token;

pub struct Lexer {
    regex: String,  // 正则字符串
    read: BufReader<File>,     // 读入的文件

    has_more: bool,      // 读入的文件是否还有未读的内容

    cur_line: Vec<char>, // 当前行
    line_num: u32,       // 当前行号
    last_char: Option<char>      // 最近读取的字符
}

impl Lexer{
    pub fn new(regex: String, read: File)->Self{
        return Lexer{
            regex: regex,  // 正则字符串
            read: BufReader::new(read),     // 读入的文件
            has_more: true,
            cur_line: Vec::new(),
            line_num: 0,
            last_char: None
        };
    }

    pub fn read(mut self){
        
        let mut word = String::from("");
        let c:Option<char>;
        loop {
            c = self.get_char();
            if(!self::is_space(c)){
                break;
            }
        }

        if(c == None){
            return None;
        }

        if(isDigit(c)){
            word.push(c.unwrap());
            loop{
                

            }
        }
        return Some(word);
    }

    fn get_char(&mut self)->Option<char>{
        if(self.last_char != None){
            return self.last_char;
        }
        if(self.cur_line.is_empty()){
            let mut line = Vec::<u8>::new();
            if(self.read.read_until(b'\n', &mut line).expect("read_until 函数调用失败") != 0) {
                // this moves the ownership of the read data to s
                // there is no allocation
                let s = String::from_utf8(line).expect("from_utf8 函数调用失败");

                for c in s.chars() {
                    self.cur_line.push(c);
                }
                // println!("当前行：{:#?}", self.cur_line);
                // this returns the ownership of the read data to buf
                // there is no allocation
                line = s.into_bytes();
                line.clear();
            }else{
                return None;
            }
        }
        return Some(self.cur_line.remove(0));
    }

    fn unget_char(&mut self, c: char){
        self.last_char = Some(c);
    }

    fn is_space(c: Option<char>)->bool{
        return c != None && c.unwrap().is_whitespace();
    }

    fn is_digit(c: Option<char>)->bool{
        return c != None && c.unwrap().is_digit();
    }
}