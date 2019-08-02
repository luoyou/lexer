use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 此处的语句只有赋值，后续需要更改为更加准确的名称
 */
#[derive(Debug)]
pub struct AssignStatementNode {
    children: Vec<Box<AstreeNode>>,
}

impl AstreeNode for AssignStatementNode{

    fn eval(&mut self, env: &mut Env)->Eval{
        let right = self.children.pop().unwrap().eval(env);
        let left  = self.children.pop().unwrap().to_string();
        env.put(left, right);
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }

}
impl AssignStatementNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return AssignStatementNode{
            children: children
        }
    }
}
