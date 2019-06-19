use std::env;
use std::fs::File;

mod core;
mod define;
use self::core::lexer::Lexer;
use self::core::ast::astree::Astree;

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
        // println!("行数：{} {}：{:?}", xxx.get_line_num(), xxx.get_type_name(), xxx.get_text());
    }
    let astree = Astree::new_node(vec![]);
    println!("子节点数量：{}", astree.get_child_num());
}