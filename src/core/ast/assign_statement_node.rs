use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 此处的语句只有赋值，后续需要更改为更加准确的名称
 */
#[derive(Debug)]
pub struct AssignStatementNode {
    children: Vec<Box<AstNode>>,
}

impl AstNode for AssignStatementNode{

    fn eval(&self, env: &mut Env)->Eval{
        // println!("{:#?}", self.children);
        let left  = self.children[0].to_string();
        let right = self.children[1].eval(env);
        // println!("{:#?}", right);
        env.put(left, right);
        return Eval::TNil;
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        return &self.children;
    }

}
impl AssignStatementNode{
    pub fn new(children: Vec<Box<AstNode>>)->Self{
        return AssignStatementNode{
            children: children
        }
    }
}
