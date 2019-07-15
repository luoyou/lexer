use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct StatementNode {
    children: Vec<Box<AstreeNode>>,
}

impl AstreeNode for StatementNode{

    fn eval(&self, env: &mut Env)->Eval{
        let left  = self.children.get(0).unwrap().to_string();
        let right = self.children.get(1).unwrap().eval(env);
        env.put(left, right);
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>{
        return &self.children;
    }

}
impl StatementNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return StatementNode{
            children: children
        }
    }
}
