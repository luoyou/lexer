use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use super::token::Token;
use super::token::TokenType;

use super::super::define::KEYWORDS;
use super::super::define::STOP_CHARS;
use super::super::define::SINGLE_CHARS;
use super::super::define::SKIP_CHARS;


pub struct Lexer {
    read: BufReader<File>,     // 读入的文件
    keyword_queue: Vec<Token>,        // 关键词队列
    cur_line: Vec<char>,       // 当前行
    cur_line_num: i32,         // 当前行号
    last_char: Option<char>    // 最近读取的字符
}

impl Lexer{

    pub fn new(read: File)->Self{
        return Lexer{
            read: BufReader::new(read),     // 读入的文件
            cur_line: Vec::new(),
            cur_line_num: 0,
            keyword_queue: vec![],
            last_char: None
        };
    }

    // 读取单词（消耗队列）
    pub fn read(&mut self)->Token{
        if self.fill_queue(0){
            return self.keyword_queue.remove(0);
            // let xxx = self.keyword_queue.pop().unwrap();
            // println!("{:#?}", xxx);
            // return xxx;
        }else{
            return Token::end();
        }
    }

    // 提取单词
    pub fn peek(&mut self, num: usize)->Token{
        if self.fill_queue(num) {
            return self.keyword_queue.get(num).cloned().unwrap();
        }else{
            return Token::end();
        }
    }

    /**
     * 当单词队列不足时，填充队列
     */
    fn fill_queue(&mut self, num: usize)->bool{
        while num >= self.keyword_queue.len() {
            let token = self.next_token();
            if token.is_end() {
                return false;
            }else{
                self.keyword_queue.push(token);
            }
        }
        return true;
    }

    /**
     * 读取下一个单词，填充队列
     */
    fn next_token(&mut self)->Token{
        let mut word = String::from("");
        let mut c:Option<char>;
        loop { // 读取字符，是空格则跳过，不是空格，保留至下方进行判断
            c = self.get_char();
            if !Lexer::is_skip_char(c) {
                break;
            } 
        }

        if c == None {  // 返回第0行，说明文件已经读取结束（文件内从第一行开始）
            return Token::end();
        } else if c.unwrap() == '\n' || c.unwrap() == ';'{
            return Token::new(self.cur_line_num, c.unwrap().to_string(), TokenType::Separater);
        } else if c.unwrap() == '/'{
            word.push(c.unwrap());
            c = self.get_char();
            if c != None && c.unwrap() == '/'{ // 达成单行注释条件
                loop {
                    word.push(c.unwrap());
                    c = self.get_char();
                    if c==None || c.unwrap() == '\n'{
                        break;
                    }
                }
                // return Token::new(self.cur_line_num, word, TokenType::Comment);
                // 注释没有必要进入token序列
                return self.next_token();
            }else{
                self.unget_char(c);
                return Token::new(self.cur_line_num, word, TokenType::Keyword);
            }
        }else if Lexer::is_number(c) {  // 是数字
            loop{ // 循环，只要读取到一直是数字，则一直往字符串中加载
                word.push(c.unwrap());  // 推入字符串中
                c = self.get_char();    // 读取下一个字符
                if !Lexer::is_number(c) {   // 不是数字，则将刚刚读取到的字符还原
                    self.unget_char(c);
                    break;
                }
            }
            return Token::new(self.cur_line_num, word, TokenType::Number);
        }else if c.unwrap() == '=' || c.unwrap() == '>' || c.unwrap() == '<' { // 是字符，= > <
            word.push(c.unwrap());
            c = self.get_char();
            if c != None && c.unwrap() == '=' {
                word.push(c.unwrap());
            }else{
                self.unget_char(c);
            }
            return Token::new(self.cur_line_num, word, TokenType::Keyword);            
        }else if c.unwrap() == '-' { // 是减号, todo 还有 -> 尚未支持
            word.push(c.unwrap());
            return Token::new(self.cur_line_num, word, TokenType::Keyword);
        }else if c.unwrap() == '*'{ // 乘号，还有注释结束符
            word.push(c.unwrap());
            c = self.get_char();
            if c != None && c.unwrap() == '/' {
                word.push(c.unwrap());
            }else{
                self.unget_char(c);
            }
            return Token::new(self.cur_line_num, word, TokenType::Keyword);
        }else if c.unwrap() == '!' {
            word.push(c.unwrap());
            c = self.get_char();
            if c != None && c.unwrap() == '='{
                word.push(c.unwrap());
            }else{
                self.unget_char(c);
            }
            return Token::new(self.cur_line_num, word, TokenType::Keyword);
        }else if Lexer::is_single_signal(c) { // 是+,{,}，直接返回
            word.push(c.unwrap());
            return Token::new(self.cur_line_num, word, TokenType::Keyword);
        }else if c.unwrap() == '\'' || c.unwrap() == '"'{
            return self.lexer_text(c);
        }else if c.unwrap() == '「' {
            return self.lexer_text(Some('」'));
        }else if c.unwrap() == '『' {
            return self.lexer_text(Some('』'));
        }else{
            word.push(c.unwrap()); // 其他的情况，统一推入字符串
            loop{
                c = self.get_char();
                if Lexer::is_stop_char(c) { // 是空格则中断
                    self.unget_char(c);
                    break; 
                }else{
                    word.push(c.unwrap());
                }
            }
        }
        if KEYWORDS.contains(&&*word){
            return Token::new(self.cur_line_num, word, TokenType::Keyword);    
        }else{
            return Token::new(self.cur_line_num, word, TokenType::Identidify);
        }
    }

    fn lexer_text(&mut self, end_char: Option<char>)->Token{
        let mut word = String::from("");
        loop {
            let c = self.get_char();
            // println!("{:#?}", c);
            if c == None {
                panic!(word + " 不是一个完整的文本标识");
            }else if c.unwrap() == '\\'{
                let next_c = self.get_char();
                if next_c == end_char {
                    word.push(next_c.unwrap());
                    continue;
                }else{
                    self.unget_char(next_c);
                }
            }else if c != end_char {
                word.push(c.unwrap());
            }else{
                break;
            }
        }
        return Token::new(self.cur_line_num, word, TokenType::Text);
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

    // 是否是可跳过字符
    fn is_skip_char(c: Option<char>)->bool{
        return c != None && SKIP_CHARS.contains(&c.unwrap());
    }

    // 是否是数字
    fn is_number(c: Option<char>)->bool{
        return c != None && c.unwrap().is_ascii_alphanumeric() && c.unwrap().is_numeric();
    }

    // // 是否是英文字母
    // fn is_letter(c: Option<char>)->bool{
    //     return c != None && c.unwrap().is_ascii_alphabetic();
    // }

    // // 是英文字母或阿拉伯数字
    // fn is_letter_or_numberic(c: Option<char>)->bool{
    //     return c != None && c.unwrap().is_ascii_alphanumeric();
    // }

    // 是单个字符关键词
    fn is_single_signal(c: Option<char>)->bool{
        return c != None && SINGLE_CHARS.contains(&c.unwrap());
    }

    // 是换行\n
    // fn is_enter(c: Option<char>)->bool{
    //     return c != None && c.unwrap() == '\n';
    // }

    fn is_stop_char(c: Option<char>)->bool{
        return c == None || STOP_CHARS.contains(&c.unwrap());
    }
}