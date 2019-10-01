use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

/**
 * 反 bool 值 例：!user
 */
#[derive(Debug)]
pub struct NotBoolNode{
    val: Box<AstNode>
}

impl AstNode for NotBoolNode{
    fn eval(&self, env: &mut Env)->Eval{
        // let op = self.children.get(0).unwrap().eval();
        // let val = self.children.pop().unwrap().eval(env);
        // println!("{:#?}", number);
        let val = self.val.eval(env);
        match val {
            Eval::TBool(n) => Eval::TBool(!n),
            _ => panic!("逻辑值异常")
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        panic!("该类型没有返回值")
    }
}
impl NotBoolNode{
    pub fn new(val: Box<AstNode>)->Self{
        NotBoolNode{
            val: val
        }
    }
}
