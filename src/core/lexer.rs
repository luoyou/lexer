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

    line_queue: Queue<String>,  // 行队列，将文件内容按行拆分形成队列
    cur_line: Option<String>, // 当前行
}

impl Lexer{
    pub fn new(regex: String, read: File)->Self{
        return Lexer{
            regex: regex,  // 正则字符串
            read: read,     // 读入的文件
            has_more: true,
            line_queue: queue![],
            cur_line: None
        };
    }

    pub fn read(mut self){
        let mut f = BufReader::new(&self.read);
        // for line in f.lines() {
        //     self.line_queue.add(line.unwrap());
        // }

        // let xxx = Token::new(10);
        // println!("查看队列结构：{:#?}", self.line_queue);
        // self.get_char();

        let mut buf = Vec::<u8>::new();
        while f.read_until(b'\n', &mut buf).expect("read_until 函数调用失败") != 0 {
            // this moves the ownership of the read data to s
            // there is no allocation
            let s = String::from_utf8(buf).expect("from_utf8 函数调用失败");
            println!("字符: {:#?}", s);
            for c in s.chars() {
                println!("字符: {}", c);
            }
            // this returns the ownership of the read data to buf
            // there is no allocation
            buf = s.into_bytes();
            buf.clear();
        }
    }

    // fn get_char(mut self){
    //     if(self.cur_line == None){
    //         self.cur_line = Some(self.line_queue.remove().unwrap());
    //     }
    //     println!("查看队列结构：{:#?}", self.cur_line);
    // }
}