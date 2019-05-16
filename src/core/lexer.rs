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

    queue: Queue<String>,
}

impl Lexer{
    pub fn new(regex: String, read: File)->Self{
        return Lexer{
            regex: regex,  // 正则字符串
            read: read,     // 读入的文件
            has_more: true,
            queue: queue![]
        };
    }

    pub fn read(self){
        let fin = BufReader::new(self.read);
        for line in fin.lines() {
            println!("行：{}", line.unwrap());
        }
        // let xxx = Token::new(10);
        println!("查看换行{}", Token::EOL);
        println!("查看换行成功");

    }
}