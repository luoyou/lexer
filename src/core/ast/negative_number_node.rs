use super::ast_node::AstNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct NegativeNumberNode{
    val: Box<AstNode>
}

impl AstNode for NegativeNumberNode{
    fn eval(&self, env: &mut Env)->Eval{
        // let op     = self.children.get(0).unwrap().eval();
        // let number = self.children.pop().unwrap().eval(env);
        // println!("{:#?}", number);
        let number = self.val.eval(env);
        match number {
            Eval::TNumber(n) => Eval::TNumber(-n),
            _ => panic!("数字异常")
        }
    }

    fn get_children(&self)->&Vec<Box<AstNode>>{
        panic!("该类型下没有子节点")
    }
}
impl NegativeNumberNode{
    pub fn new(val: Box<AstNode>)->Self{
        NegativeNumberNode{
            val: val
        }
    }
}
