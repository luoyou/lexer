use super::super::ast::eval::Eval;


pub fn sys_fn_exist(fn_name: &str)->bool{
    let fn_list:[&str; 3] = ["输出", "输出行", "文本拼接"];
    return fn_list.contains(&fn_name);
}

pub fn sys_fn_call(fn_name: String, param_list: &mut Vec<Eval>)->Eval{
    match &*fn_name{
        "输出" => output(param_list, false),
        "输出行" => output(param_list, true),
        "文本拼接" => str_append(param_list),
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

fn str_append(param_list: &Vec<Eval>)->Eval{
    if param_list.len() < 2 {
        panic!("文本拼接必须两个参数以上");
    }
    let mut ret = Eval::TText("".to_string());
    for param in param_list {
        ret = ret.connect(param);
    }
    ret
}