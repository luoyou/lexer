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

    cur_line: Vec<u8>, // 当前行
    line_num: u32,     // 当前行号
}

impl Lexer{
    pub fn new(regex: String, read: File)->Self{
        return Lexer{
            regex: regex,  // 正则字符串
            read: read,     // 读入的文件
            has_more: true,
            cur_line: Vec::<u8>::new()，
            line_num: 0
        };
    }

    pub fn read(mut self){
        self.get_char();
    }

    fn get_char(mut self){
        let mut f = BufReader::new(&self.read);

        while f.read_until(b'\n', &mut self.cur_line).expect("read_until 函数调用失败") != 0 {
            // this moves the ownership of the read data to s
            // there is no allocation
            let s = String::from_utf8(self.cur_line).expect("from_utf8 函数调用失败");
            println!("行长度: {}", s.len());
            for c in s.chars() {
                println!("字符: {}", c);
            }
            // this returns the ownership of the read data to buf
            // there is no allocation
            self.cur_line = s.into_bytes();
            self.cur_line.clear();
        }
    }
}