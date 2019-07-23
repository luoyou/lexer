use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ParamListNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for ParamListNode{

    fn eval(&self, env: &mut Env)->Eval{
        Eval::TNil
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl ParamListNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return ParamListNode{
            children: children
        }
    }
}
