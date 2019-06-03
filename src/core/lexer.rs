use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::token::IdToken;
use super::token::Token;
use super::token::Word;

pub struct Lexer {
    read: BufReader<File>,     // 读入的文件
    cur_line: Vec<char>, // 当前行
    cur_line_num: u32,       // 当前行号
    last_char: Option<char>      // 最近读取的字符
}

impl Lexer{
    // const Keyword = Vec![];

    pub fn new(read: File)->Self{
        return Lexer{
            read: BufReader::new(read),     // 读入的文件
            cur_line: Vec::new(),
            cur_line_num: 0,
            last_char: None
        };
    }

    pub fn read(&mut self)->impl Token{
        let mut word = String::from("");
        let mut c:Option<char>;
        loop { // 读取字符，是空格则跳过，不是空格，保留至下方进行判断
            c = self.get_char();
            if !Lexer::is_space(c) {
                break;
            }
        }

        if c == None {  // 返回第0行，说明文件已经读取结束（文件内从第一行开始）
            return IdToken::new(Word::EOF, word);
        }else if Lexer::is_number(c) {  // 是数字
            loop{ // 循环，只要读取到一直是数字，则一直往字符串中加载
                word.push(c.unwrap());  // 推入字符串中
                c = self.get_char();    // 读取下一个字符
                if !Lexer::is_number(c) {   // 不是数字，则将刚刚读取到的字符还原
                    self.unget_char(c);
                    break;
                }
            }
        }else if Lexer::is_letter(c) { // 是英文字母
            loop {
                word.push(c.unwrap()); // 逻辑同数字
                c = self.get_char();
                if !Lexer::is_letter_or_numberic(c) { // 是字母或数字，这两者可以继续组成单词
                    self.unget_char(c);
                    break;
                }
            }
        }else if c.unwrap() == '=' || c.unwrap() == '>' || c.unwrap() == '<' { // 是字符，= > <
            word.push(c.unwrap());
            c = self.get_char();
            if c != None && c.unwrap() == '=' {
                word.push(c.unwrap());
            }else{
                self.unget_char(c);
            }
        }else if c.unwrap() == '-' { // 是减号，需要支持负数形式
            word.push(c.unwrap());
            loop{
                c = self.get_char();
                if Lexer::is_number(c) {
                    word.push(c.unwrap());
                }else{
                    self.unget_char(c);
                    break;
                }
            }
        }else if Lexer::is_single_signal(c) { // 是+,{,}，直接返回
            word.push(c.unwrap());
        }else{
            word.push(c.unwrap()); // 其他的情况，统一推入字符串
            loop{
                c = self.get_char();
                if Lexer::is_space(c) || Lexer::is_enter(c) { // 是空格则中断
                    self.unget_char(c);
                    break; 
                }else{
                    word.push(c.unwrap());
                }
            }
        }

        return IdToken::new(self.cur_line_num, word);
    }

    fn get_char(&mut self)->Option<char>{
        if self.last_char != None {
            let c = self.last_char;
            self.last_char = None;
            return c;
        }
        if self.cur_line.is_empty() {
            let mut line = Vec::<u8>::new();
            if self.read.read_until(b'\n', &mut line).expect("read_until 函数调用失败") != 0 {
                self.cur_line_num = self.cur_line_num + 1;
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
                // println!("字符：None");
                return None;
            }
        }
        let c = self.cur_line.remove(0);
        // println!("字符：{}", c);
        return Some(c);
    }

    fn unget_char(&mut self, c: Option<char>){
        self.last_char = c;
    }

    // 是否是空格
    fn is_space(c: Option<char>)->bool{
        return c != None && c.unwrap() == ' ';
    }

    // 是否是数字
    fn is_number(c: Option<char>)->bool{
        return c != None && c.unwrap().is_ascii_alphanumeric() && c.unwrap().is_numeric();
    }

    // 是否是英文字母
    fn is_letter(c: Option<char>)->bool{
        return c != None && c.unwrap().is_ascii_alphabetic();
    }

    // 是英文字母或阿拉伯数字
    fn is_letter_or_numberic(c: Option<char>)->bool{
        return c != None && c.unwrap().is_ascii_alphanumeric();
    }

    // 是单个字符关键词
    fn is_single_signal(c: Option<char>)->bool{
        let unwrap_c = c.unwrap();
        return unwrap_c == '+' || unwrap_c == '{' || unwrap_c == '}' || unwrap_c == '\r' || unwrap_c == '\n';
    }

    // 是换行\n
    fn is_enter(c: Option<char>)->bool{
        return c != None && c.unwrap() == '\n';
    }
}