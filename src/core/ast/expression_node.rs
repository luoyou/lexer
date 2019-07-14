use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ExpressionNode {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for ExpressionNode{

    fn eval(&self, env: &mut Env)->Eval{
        let left  = self.children.get(0).unwrap().eval(env);
        let op    = self.children.get(1).unwrap().eval(env);
        let right = self.children.get(2).unwrap().eval(env);
        left.cal(op, right)
    }

    fn location(&self)->String{
        for node in &self.children{
            let s = node.location();
            if s != "".to_string() {
                return s;
            }
        }
        return "".to_string();
    }

    fn to_string(&self)->String{
        let mut builder = String::from("(");
        let mut sep = "";
        for node in &self.children{
            builder.push_str(sep);
            sep = " ";
            builder.push_str(&node.to_string());
        }
        builder.push(')');
        return builder;
    }

}

impl ExpressionNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return ExpressionNode{
            children: children
        }
    }
}
