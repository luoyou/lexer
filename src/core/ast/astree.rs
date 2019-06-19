use super::super::token::Token;

#[derive(PartialEq)]
enum AstreeType {
    Root,   // 根节点
    Node,   // 枝节点
    Leaf    // 叶节点
}

// 抽象语法树结构
pub struct Astree {
    children_num: u8,
    token: Option<Token>,
    children: Vec<Astree>,
    node_type: AstreeType
}

impl Astree {
    // 新建枝节点
    pub fn new_node(children: Vec<Astree>)->Self{
        return Astree{
            children_num: 0,
            token: None,
            children: children,
            node_type: AstreeType::Node
        }
    }

    // 新建叶节点
    pub fn new_leaf(token: Token)->Self{
        return Astree{
            children_num: 0,
            token: Some(token),
            children: vec![],
            node_type: AstreeType::Leaf
        }
    }

    // 获取某个子节点
    fn child(&self, i: u8)->&Astree{
        if i == 0 {
            println!("aaa");
        }
        return self;
    }

    // 获取所有子节点
    fn children(&self)->&Vec<Astree>{
        return &self.children;
    }

    // 获取子节点数量
    pub fn get_child_num(&self)->u8{
        return self.children_num;
    }

    // 读取token的位置
    pub fn location(&self)->Option<String>{
        if self.is_leaf() {
            let mut location = "位于".to_string();
            let token = self.token.unwrap();
            // location.push_str(&token.get_line_num().to_string());
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
    fn is_leaf(&self)->bool{
        return self.node_type == AstreeType::Leaf;
    }

    // 返回字面量
    // pub fn to_string(&self)->String{
    //     if self.is_leaf() {
    //         return self.token.unwrap().get_text().to_string();
    //     }
    //     let mut builder = String::from("(");
    //     let mut sep = "";
    //     for node in &self.children{
    //         builder.push_str(sep);
    //         sep = " ";
    //         builder.push_str(&node.to_string());
    //     }
    //     builder.push(')');
    //     return builder;
    // }
    
}