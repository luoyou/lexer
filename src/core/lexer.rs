use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Lexer {
    pub regex: String,  // 正则字符串
    pub read: File,     // 读入的文件
    has_more: bool ,     // 读入的文件是否还有未读的内容

//    queue: array,
}

impl Lexer{
    pub fn new(&self, regex: String, read: File)->self{

        return self;
    }

    pub fn read(&self){
        let fin = BufReader::new(&self.read);
        for line in fin.lines() {
            println!("行：{}", line.unwrap());
        }
    }
}