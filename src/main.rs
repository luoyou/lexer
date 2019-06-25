use std::env;
use std::fs::File;

mod core;
mod define;
use self::core::lexer::Lexer;
use self::core::parse::Parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // test_lexer();
    test_ast_parse();
}

// 词法分析测试
fn test_lexer(){
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

// 语法分析测试
fn test_ast_parse(){
    let file = File::open("stone/expr.txt").expect("未能打开文件");
    let mut lexer = Lexer::new(file);
    let mut parse = Parse::new(lexer);
    let ast = parse.expression();
    println!("语法分析结果：{:#?}", ast.eval());
}
