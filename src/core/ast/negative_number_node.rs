use super::astree::AstreeNode;
use super::eval::Eval;
use super::env::Env;

#[derive(Debug)]
pub struct NegativeNumberNode{
    children: Vec<Box<AstreeNode>>
}

impl AstreeNode for NegativeNumberNode{
    fn eval(&self, env: &mut Env)->Eval{
        // let op     = self.children.get(0).unwrap().eval();
        let number = self.children.get(1).unwrap().eval(env);
        // println!("{:#?}", number);
        match number {
            Eval::TNumber(n) => Eval::TNumber(-n),
            _ => panic!("数字异常")
        }
    }

    fn location(&self)->String{
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
impl NegativeNumberNode{
    pub fn new(children: Vec<Box<AstreeNode>>)->Self{
        NegativeNumberNode{
            children: children
        }
    }
}
