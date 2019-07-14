use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct ProgramRoot {
    children: Vec<Box<AstreeNode>>
}


impl AstreeNode for ProgramRoot{

    /**
     * 从根节点进行求值
     */
    fn eval(&self, env: &mut Env)->Eval{
        for child in &self.children{
            child.eval(env);
        } 
        return Eval::TNil;
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

impl ProgramRoot{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        return ProgramRoot{
            children: children
        }
    }

    pub fn push(&mut self, node: Box<AstreeNode>){
        self.children.push(node);
    }
}
