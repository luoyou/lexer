use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct WhileStatementNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for WhileStatementNode{

    /**
     *
     */
    fn eval(&self, env: &mut Env)->Eval{
        let condition  = self.children.get(0).unwrap();
        let block = self.children.get(1).unwrap();
        loop {
            if condition.eval(env) == Eval::TBool(true) {
                block.eval(env);
            }else{
                break;
            }
        }
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }
}

impl WhileStatementNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return WhileStatementNode{
            children: children
        }
    }
}
