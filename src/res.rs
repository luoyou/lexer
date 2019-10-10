use std::process;

pub struct Res{

}

impl Res {
    pub fn error(text: &str){
        println!("{}", text);
        process::exit(1);
    }
}