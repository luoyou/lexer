use super::super::ast::eval::Eval;


pub fn sys_fn_exist(fn_name: &str)->bool{
    let fn_list:[&str; 2] = ["输出", "输出行"];
    return fn_list.contains(&fn_name);
}

pub fn sys_fn_call(fn_name: String, param_list: &mut Vec<Eval>)->Eval{
    match &*fn_name{
        "输出" => output(param_list, false),
        "输出行" => output(param_list, true),
        _ => panic!("函数不存在")
    }
}

fn output(param_list: &mut Vec<Eval>, is_line: bool)->Eval{
    if param_list.len() < 1 {
        panic!("输出函数必须包含一个参数");
    }
    let val = match &param_list[0]{
        Eval::TNil => "nil".to_string(),
        Eval::TBool(val) => val.to_string(),
        Eval::TNumber(val) => val.to_string(),
        Eval::TText(val) => val.to_string(),
    };
    if is_line {
        println!("{}", val);
    }else{
        print!("{}", val);
    }
    Eval::TNil
}

fn str_append(str1: Eval, str2: Eval)->Eval{
    
}