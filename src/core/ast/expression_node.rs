use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ExpressionNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for ExpressionNode{

    fn eval(&self, env: &mut Env)->Eval{
        let left  = self.children.get(0).unwrap().eval(env);
        let op    = self.children.get(1).unwrap().eval(env);
        let right = self.children.get(2).unwrap().eval(env);
        left.cal(op, right)
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
