use std::env;
use std::fs::File;

mod core;
use self::core::lexer::Lexer;
mod define;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file = File::open("stone/demo.txt").expect("未能打开文件");
    let mut lexer = Lexer::new(file);
    let mut i = 0;
    loop{
        let xxx = lexer.peek(i);
        if xxx.get_line_num() == 0 {
            break;
        }
        i = i + 1;
        println!("行数：{} {}：{:?}", xxx.get_line_num(), xxx.get_type_name(), xxx.get_text());
    }
}