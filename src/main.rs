use std::env;
use std::fs::File;
use std::process;
use std::path::Path;

mod core;
mod define;
mod res;
use self::core::lexer::Lexer;
use self::core::parse::Parse;
use self::core::ast::env::Env;
use self::res::Res;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        Res::error("请输入要运行的程序文件路径");
    }
    // test_lexer();
    let file_path = args.remove(1);
    test_ast_parse(&file_path);
}

// 词法分析测试
// fn test_lexer(){
//     let file = File::open("stone/demo.txt").expect("未能打开文件");
//     let mut lexer = Lexer::new(file);
//     let mut i = 0;
//     loop{
//         let xxx = lexer.peek(i);
//         if xxx.get_line_num() == 0 {
//             break;
//         }
//         i = i + 1;
//         println!("行数：{} {}：{:?}", xxx.get_line_num(), xxx.get_type_name(), xxx.get_text());
//     }
// }

// 语法分析测试
fn test_ast_parse(file_path: &str){
    if !Path::new(file_path).exists() {
        Res::error("文件不存在");
    }
    let file = File::open(file_path).expect("未能打开文件");
    let lexer = Lexer::new(file);
    let mut parse = Parse::new(lexer);
    let mut env = Env::new();
    let mut ast = parse.program(&mut env);
    let eval_val = ast.eval(&mut env);
    // println!("语法分析结果：{:#?}", eval_val);
    // println!("抽象语法树：{:#?}", ast);
    // println!("语法分析结果：{:#?}", ast);
    // println!("上下文变量：{:#?}", env);
}
