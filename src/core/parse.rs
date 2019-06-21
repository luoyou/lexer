use super::lexer::Lexer;

use super::ast::astree::Astree;

pub struct Parse{
    lexer: Lexer
}


// factor:     NUMBER | "(" expression ")"
// term:       factor { ("*" | "/") factor }
// expression: term   { ("+" | "-") term }

impl Parse{
    pub fn new(lexer: Lexer)->Self{
        return Parse{
            lexer: lexer
        }
    }

    pub fn expression(&mut self)->Astree{
        let mut left = self.term();
        // println!("表达式：{:#?}", left);
        while self.is_token("+") || self.is_token("-") {
            let op = Astree::new_leaf(self.lexer.read());
            let right = self.term();
            left = Astree::new_node(vec![left, op, right]);
        }
        return left;
    }

    pub fn term(&mut self)->Astree{
        let mut left = self.factor();
        // println!("项：{:#?}", left);
        while self.is_token("*") || self.is_token("/") {
            let op = Astree::new_leaf(self.lexer.read());
            let right = self.factor();
            left = Astree::new_node(vec![left, op, right]);
        }
        return left;
    }

    pub fn factor(&mut self)->Astree{
        if self.is_token("(") {
            self.token("(");
            let e = self.expression();
            self.token(")");
            return e;
        }else{
            let token = self.lexer.read();
            if token.is_number() {
                let num_leaf = Astree::new_leaf(token);
                return num_leaf;
            }else{
                panic!("读取到的是非数字字符");
            }
        }
    }

    // 消耗单词
    fn token(&mut self, name: &str){
        let token = self.lexer.read();
        if token.get_text() != name {
            panic!("读取单词错误");
        }
    }

    // 预读单词
    fn is_token(&mut self, name: &str)->bool{
        let token = self.lexer.peek(0);
        // println!("预读：{:#?}, name:{}", token, name);
        return token.get_text() == name;
    }

}