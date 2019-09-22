use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 非 bool值
 */
#[derive(Debug)]
pub struct NotBoolNode{
    children: Vec<Box<AstNode>>
}

impl AstNode for NotBoolNode{
    fn eval(&mut self, env: &mut Env)->Eval{
        // let op     = self.children.get(0).unwrap().eval();
        let val = self.children.pop().unwrap().eval(env);
        // println!("{:#?}", number);
        match val {
            Eval::TBool(n) => Eval::TBool(!n),
            _ => panic!("逻辑值异常")
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}
impl NotBoolNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        NotBoolNode{
            children: children
        }
    }
}
