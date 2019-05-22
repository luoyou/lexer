extern crate queues;

use queues::*;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::token::Token;

pub struct Lexer {
    regex: String,  // 正则字符串
    read: File,     // 读入的文件

    has_more: bool,      // 读入的文件是否还有未读的内容

    // cur_line: Chars, // 当前行
    line_num: u32,     // 当前行号
}

impl Lexer{
    pub fn new(regex: String, read: File)->Self{
        return Lexer{
            regex: regex,  // 正则字符串
            read: read,     // 读入的文件
            has_more: true,
            // cur_line: "".chars(),
            line_num: 0
        };
    }

    pub fn read(mut self){
        // self.get_char();
        self.test()
    }

    // fn get_char(mut self)->Option<char>{
    //     let mut f = BufReader::new(&self.read);
    //     if !self.has_more {
    //         return None;
    //     } else {
    //         let mut line = Vec::<u8>::new();
    //         if f.read_until(b'\n', &mut line).expect("read_until 函数调用失败") != 0 {
    //             // this moves the ownership of the read data to s
    //             // there is no allocation
    //             let mut xxx = String::from_utf8(line).expect("from_utf8 函数调用失败").chars();
    //             return xxx.next();
    //         }else{
    //             return None;
    //         }
    //     }
    // }

    fn test(self){
        let mut f = BufReader::new(&self.read);
        let mut line = Vec::<u8>::new();
        if f.read_until(b'\n', &mut line).expect("read_until 函数调用失败") != 0 {
            // this moves the ownership of the read data to s
            // there is no allocation
            let mut xxx = String::from_utf8(line).expect("from_utf8 函数调用失败");
            let yyy = xxx.chars();
            println!("{:?}", yyy);
        }        
    }
}