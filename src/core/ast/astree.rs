use std::fmt::Debug;  
use super::eval::Eval;


pub trait AstreeNode: Debug{
    fn eval(&self)->Eval{
        panic!("本类型不支持求值")
    }

    // fn child(&self, num: i32)->&Self;

    // fn children(&self)->Vec<&Self>{
    //     return vec![];
    // }

    fn location(&self)->String;

    fn to_string(&self)->String;
}