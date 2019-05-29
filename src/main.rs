use std::env;
use std::fs::File;

mod core;
use self::core::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file = File::open("stone/demo.txt").expect("未能打开文件");
    let mut lexer = Lexer::new(file);
    loop{
        let xxx = lexer.read();
        if xxx.get_line_num() == 0 {
            break;
        }
        println!("读取单词：{:?}", xxx);
    }
}