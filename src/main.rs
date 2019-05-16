use std::env;
use std::fs::File;

mod core;
use self::core::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let file = File::open("stone/demo.txt").expect("未能打开文件");
    let lexer = Lexer::new(String::from("123"), file);
    lexer.read();
}