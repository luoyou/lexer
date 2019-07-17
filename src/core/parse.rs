use super::lexer::Lexer;

use super::ast::astree::AstreeNode;
use super::ast::program_root::ProgramRoot;
use super::ast::expression_node::ExpressionNode;
use super::ast::negative_number_node::NegativeNumberNode;
use super::ast::number_leaf::NumberLeaf;
use super::ast::op_leaf::OpLeaf;
use super::ast::identidify_leaf::IdentidifyLeaf;
use super::ast::statement_node::StatementNode;
use super::ast::if_statement_node::IfStatementNode;
use super::ast::while_statement_node::WhileStatementNode;
use super::ast::block_node::BlockNode;

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

    pub fn program(&mut self)->Box<AstreeNode>{
        let mut program = ProgramRoot::new(vec![]);
        loop {
            if self.is_sep_token() {
                self.token_sep();
            }else if self.is_end_token(){
                break;
            }else{
                program.push(self.statement());
            }
        }
        return Box::new(program);
    }

    fn statement(&mut self)->Box<AstreeNode>{
        if self.is_token("if") || self.is_token("如果") {
            return self.if_statement();
        }else if self.is_token("while") || self.is_token("当"){
            return self.while_statement();
        }else if self.next_is_token(1, "="){
            let left = self.indentidify();
            self.token("=");
            let right = self.expression();
            let statement = StatementNode::new(vec![left, right]);
            return Box::new(statement);
        }else{
            return self.expression();
        }
    }

    fn if_statement(&mut self)->Box<AstreeNode>{
        self.tokens(vec!["if", "如果"]);
        let mut statements: Vec<Box<AstreeNode>> = Vec::new();
        statements.push(self.expression());
        statements.push(self.block()); 
        if self.is_token("else") || self.is_token("否则") {
            self.tokens(vec!["else", "否则"]);
            statements.push(self.block());
        }
        return Box::new(IfStatementNode::new(statements));
    }

    fn while_statement(&mut self)->Box<AstreeNode>{
        self.tokens(vec!["while", "当"]);
        let expr  = self.expression();
        let block = self.block();
        return Box::new(WhileStatementNode::new(vec![expr, block]));
    }

    fn block(&mut self)->Box<AstreeNode>{
        self.token("{");
        let mut statements: Vec<Box<AstreeNode>> = Vec::new();
        loop {
            if self.is_token("}") {
                self.token("}");
                break;
            }else if self.is_sep_token() {
                self.token_sep(); // 遇到分隔符则消耗掉
            }else{
                statements.push(self.statement());
            }
        }
        return Box::new(BlockNode::new(statements));
    }

    fn indentidify(&mut self)->Box<AstreeNode>{
        let token = self.lexer.read();
        if token.is_identidify() {
            let id_leaf = IdentidifyLeaf::new(token);
            return Box::new(id_leaf);
        }else{
            panic!("读取到的是非标识符");
        }
    }

    fn expression(&mut self)->Box<AstreeNode>{
        let mut left = self.term();
        // println!("表达式：{:#?}", left);
        while self.is_tokens(vec!["+", "-", "==", ">", ">=", "<", "<="]) {
            let op = OpLeaf::new(self.lexer.read());
            let right = self.term();
            let expr_node = ExpressionNode::new(
                vec![left, Box::new(op), right]
            );
            left = Box::new(expr_node);
        }
        return left;
    }

    fn term(&mut self)->Box<AstreeNode>{
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

    fn factor(&mut self)->Box<AstreeNode>{
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
        }else if self.is_token("!"){
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

    /**
     * 当多个单词同义的时候，能直接消除
     */
    fn tokens(&mut self, names: Vec<&str>){
        let mut tokened = false;
        for name in &names {
            if self.is_token(name) {
                self.token(name);
                tokened = true;
                break;
            }
        }
        if tokened == false {
            panic!("读取单词错误！");
        }
    }

    // 消耗单词
    fn token(&mut self, name: &str){
        let token = self.lexer.read();
        if token.get_text() != name {
            panic!(token.get_text().to_string() + "读取单词错误，此处应为：" + name);
        }
    }

    /**
     * 预读多个单词，判断是该单词
     */
    fn is_tokens(&mut self, names: Vec<&str>)->bool{
        for name in &names {
            if self.is_token(name) {
                return true;
            }
        }
        return false;
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

    /**
     * 下一个字符是分隔符 \n 或 ; 或文件结束
     */
    fn is_sep_token(&mut self)->bool{
        let token = self.lexer.peek(0);
        return token.is_serarater();
    }

    /**
     * 消耗分隔符token
     */
    fn token_sep(&mut self){
        let token = self.lexer.read();
        if !token.is_serarater() {
            panic!(token.get_text().to_string() + "不是 \n ; 或文件结束符");
        }
    }

    /**
     * 是文件终结符
     */
    fn is_end_token(&mut self)->bool{
        let token = self.lexer.peek(0);
        return token.is_end();
    }

    /**
     * 消耗文件终结符token
     */
    fn token_end(&mut self){
        let token = self.lexer.read();
        if !token.is_end() {
            panic!("不是文件结束符");
        }
    }

}