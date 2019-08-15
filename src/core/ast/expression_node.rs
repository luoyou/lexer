use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ExpressionNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for ExpressionNode{

    fn eval(&mut self, env: &mut Env)->Eval{
        let left  = self.children[0].eval(env);
        let op    = self.children[1].eval(env);
        let right = self.children[2].eval(env);
        op.cal(left, right)
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl ExpressionNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return ExpressionNode{
            children: children
        }
    }
}
