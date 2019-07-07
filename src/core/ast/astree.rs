use super::super::token::Token;
use super::eval::Eval;
// use super::eval::BasicType;


pub trait AstreeNode{
    fn eval(&self)->Eval;

    fn child(&self, num: i32)->Self;

    fn children(&self)->Vec<Self>{
        return vec![];
    }
}

struct ExprNode {
    children: Vec<AstreeNode>
}

impl AstreeNode for ExprNode{
    pub fn new(children: Vec<AstreeNode>)->Self{
        return ExprNode{
            children: children
        }
    }

    fn child(&self, num: i32)->Option<AstreeNode>{
        return 
    }

    fn eval(&self)->Eval{

    }
}

#[derive(PartialEq, Debug)]
pub enum AstreeType {
    Root,   // 根节点
    Node,   // 枝节点
    Leaf    // 叶节点
}

// 抽象语法树结构
#[derive(Debug)]
pub struct Astree {
    token: Token,
    children: Vec<Astree>,
    node_type: AstreeType
}

impl Astree {
    // 新建枝节点
    pub fn new_node(children: Vec<Astree>)->Self{
        return Astree{
            token: Token::nothing(),
            children: children,
            node_type: AstreeType::Node
        }
    }

    // 新建叶节点
    pub fn new_leaf(token: Token)->Self{
        return Astree{
            token: token,
            children: vec![],
            node_type: AstreeType::Leaf
        }
    }

    // 获取某个子节点
    pub fn child(&self, i: u8)->&Astree{
        if i == 0 {
            println!("aaa");
        }
        return self;
    }

    // 获取所有子节点
    pub fn children(&self)->&Vec<Astree>{
        return &self.children;
    }

    pub fn token(&self)->&Token{
        return &self.token;
    }

    // 获取子节点数量
    pub fn get_child_num(&self)->usize{
        return self.children.len();
    }

    // 读取token的位置
    // 递归调用子节点的 location ，一直调用到叶节点为止
    pub fn location(&self)->Option<String>{
        if self.is_leaf() {
            let mut location = "位于".to_string();
            if self.token.has_location(){
                location.push_str(&self.token.get_line_num().to_string())
            }else{
                location = "未找到错误行数".to_string()
            }
            
            return Some(location);
        }else{
            for node in &self.children{
                let s = node.location();
                if s != None {
                    return s;
                }
            }
            return None;
        }
    }

    // 是否是叶节点
    pub fn is_leaf(&self)->bool{
        return self.node_type == AstreeType::Leaf;
    }

    // 返回字面量
    // 调用子节点的 to_string, 一直到叶节点为止
    pub fn to_string(&self)->String{
        if self.is_leaf() {
            return self.token.get_text().to_string();
        }
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

    pub fn eval(&self)->Eval{
        return Eval::new(self);
    }

    // 对节点进行求值
    // pub fn eval(&self)->i32{
    //     if self.is_leaf() {
    //         if self.token.is_number() {
    //             return self.token.get_text().parse().unwrap();
    //         }else{
    //             panic!(self.token.get_text().to_string() + " 类型错误");
    //         }
    //     }else{
    //         let left_ast = self.children.get(0);
    //         let left = left_ast.unwrap().eval();
            
    //         let op_ast = self.children.get(1);
    //         let op = op_ast.unwrap().token.get_text();

    //         let right_ast = self.children.get(2);
    //         let right = right_ast.unwrap().eval();
    //         // println!("左值：{},右值：{}", left, right);
    //         match op {
    //             "+" => {
    //                 return left + right;
    //             },
    //             "-" => {
    //                 return left - right;
    //             },
    //             "*" => {
    //                 return left * right;
    //             },
    //             "/" => {
    //                 return left / right;
    //             },
    //             _ => panic!("操作符错误")
    //         }
    //     }
    // }
    
}