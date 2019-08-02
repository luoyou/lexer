use std::fmt::Debug;  
use super::eval::Eval;
use super::env::Env;


pub trait AstreeNode: Debug{
    fn eval(&mut self, _: &mut Env)->Eval{
        panic!("本类型不支持求值")
    }

    fn call(&mut self, _: &mut Vec<Eval>, _: &mut Env)->Eval{
        panic!("本类型不支持调用")
    }

    // fn child(&self, num: i32)->&Self;

    // fn children(&self)->Vec<&Self>{
    //     return vec![];
    // }

    /**
     * 获取标识符的名字，函数名，变量名，类名
     */
    fn get_id_name(&self)->String{
        panic!("该类型不支持获取标识名");
    }

    fn get_children(&self)->&Vec<Box<AstreeNode>>;

    fn location(&self)->String{
        let nodes = self.get_children();
        for node in nodes{
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
        let nodes = self.get_children();
        for node in nodes{
            builder.push_str(sep);
            sep = " ";
            builder.push_str(&node.to_string());
        }
        builder.push(')');
        return builder;
    }
}