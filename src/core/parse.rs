use super::lexer::Lexer;

use super::ast::astree::AstreeNode;
use super::ast::expression_node::ExpressionNode;
use super::ast::negative_number_node::NegativeNumberNode;
use super::ast::number_leaf::NumberLeaf;
use super::ast::op_leaf::OpLeaf;
use super::ast::identidify_leaf::IdentidifyLeaf;
use super::ast::statement_node::StatementNode;

pub struct Parse{
    lexer: Lexer
}


// factor:     NUMBER | "(" expression ")" | - factor
// term:       factor { ("*" | "/") factor }
// expression: term   { ("+" | "-") term }

impl Parse{
    pub fn new(lexer: Lexer)->Self{
        return Parse{
            lexer: lexer
        }
    }

    pub fn statement(&mut self)->Box<AstreeNode>{
        if self.next_is_token(1, "="){
            let left = self.indentidify();
            self.token("=");
            let right = self.expression();
            let statement = StatementNode::new(vec![left, right]);
            return Box::new(statement);
        }else{
            return self.expression();
        }
    }

    pub fn indentidify(&mut self)->Box<AstreeNode>{
        let token = self.lexer.read();
        if token.is_identidify() {
            let id_leaf = IdentidifyLeaf::new(token);
            return Box::new(id_leaf);
        }else{
            panic!("读取到的是非标识符");
        }
    }

    pub fn expression(&mut self)->Box<AstreeNode>{
        let mut left = self.term();
        // println!("表达式：{:#?}", left);
        while self.is_token("+") || self.is_token("-") {
            let op = OpLeaf::new(self.lexer.read());
            let right = self.term();
            let expr_node = ExpressionNode::new(
                vec![left, Box::new(op), right]
            );
            left = Box::new(expr_node);
        }
        return left;
    }

    pub fn term(&mut self)->Box<AstreeNode>{
        let mut left = self.factor();
        // println!("项：{:#?}", left);
        while self.is_token("*") || self.is_token("/") || self.is_token("%") {
            let op = OpLeaf::new(self.lexer.read());
            let right = self.factor();
            let expr_node = ExpressionNode::new(
                vec![left, Box::new(op), right]
            );
            left = Box::new(expr_node);
        }
        return left;
    }

    pub fn factor(&mut self)->Box<AstreeNode>{
        if self.is_token("(") {
            self.token("(");
            let e = self.expression();
            self.token(")");
            return e;
        }else if self.is_token("-"){
            let op = OpLeaf::new(self.lexer.read());
            let factor = self.factor();
            let negative_node = NegativeNumberNode::new(
                vec![Box::new(op), factor]
            );
            return Box::new(negative_node);
        }else{
            let token = self.lexer.read();
            // println!("{:#?}", token);
            if token.is_number() {
                let num_leaf = NumberLeaf::new(token);
                return Box::new(num_leaf);
            }else if token.is_identidify(){
                return Box::new(IdentidifyLeaf::new(token));
            }else{
                panic!("读取到的是非数字字符");
            }
        }
    }

    // 消耗单词
    fn token(&mut self, name: &str){
        let token = self.lexer.read();
        if token.get_text() != name {
            panic!(token.get_text().to_string() + "读取单词错误，此处应为：" + name);
        }
    }

    // 预读单词
    fn is_token(&mut self, name: &str)->bool{
        return self.next_is_token(0, name);
    }

    fn next_is_token(&mut self, num: usize, name: &str)->bool{
        let token = self.lexer.peek(num);
        // println!("{}.{}:{:#?}", num, name, token);
        return token.get_text() == name;
    }

}