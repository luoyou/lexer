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
        let condition  = self.children[0].eval(env);
        if condition == Eval::TBool(true) {
            self.children[1].eval(env);
        }else{
            if self.children.len() == 3 { // 已经去掉判断项和true项，所以如果还有false项，那么数组的长度应该为1
                self.children[2].eval(env);
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
