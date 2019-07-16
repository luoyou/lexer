use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct IfStatementNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for IfStatementNode{

    /**
     */
    fn eval(&self, env: &mut Env)->Eval{
        let condition  = self.children.get(0).unwrap().eval(env);
        let true_block = self.children.get(1).unwrap();
        if condition == Eval::TBool(true) {
            true_block.eval(env);
        }else{
            if self.children.len() == 3 {
                let false_block = self.children.get(2).unwrap();
                false_block.eval(env);
            }
        }
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl IfStatementNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return IfStatementNode{
            children: children
        }
    }
}
