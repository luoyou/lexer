use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct NegativeNumberNode{
    children: Vec<Box<AstNode>>
}

impl AstNode for NegativeNumberNode{
    fn eval(&mut self, env: &mut Env)->Eval{
        // let op     = self.children.get(0).unwrap().eval();
        let number = self.children.pop().unwrap().eval(env);
        // println!("{:#?}", number);
        match number {
            Eval::TNumber(n) => Eval::TNumber(-n),
            _ => panic!("数字异常")
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}
impl NegativeNumberNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        NegativeNumberNode{
            children: children
        }
    }
}
