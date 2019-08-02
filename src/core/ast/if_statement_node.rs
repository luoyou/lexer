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
    fn eval(&mut self, env: &mut Env)->Eval{
        let condition  = self.children.remove(0).eval(env);
        let mut true_block = self.children.remove(0);
        if condition == Eval::TBool(true) {
            true_block.eval(env);
        }else{
            if self.children.len() == 1 { // 已经去掉判断项和true项，所以如果还有false项，那么数组的长度应该为1
                let mut false_block = self.children.remove(0);
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
