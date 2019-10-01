use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ExpressionNode {
    children: Vec<Box<AstNode>>
}


impl AstNode for ExpressionNode{

    fn eval(&self, env: &mut Env)->Eval{
        let left  = self.children[0].eval(env);
        let op    = self.children[1].eval(env);
        let right = self.children[2].eval(env);
        op.cal(left, right)
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }
}

impl ExpressionNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        return ExpressionNode{
            children: children
        }
    }
}
