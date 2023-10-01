
extern crate regex;


use regex::Regex;

fn main(){
    let reg = Regex::new(r"[prt]ain").unwrap();
    let reg1 = Regex::new(r"[prt].ain").unwrap();

    let text = "main spain none rrrain";

    println!("The text has a match {:?}",reg.is_match(text));
    println!("The matched text is {:?}",reg.find(text));

    for cap in reg.captures_iter(text){
        println!("match: {:?}",&cap[0]);
    }

    for cap in reg1.captures_iter(text){
        println!("match: {:?}",&cap[0]);
    }

    let reg_gray = Regex::new(r"gr[ae]y").unwrap();
    let text = "gray grey graye";

    for cap in reg_gray.captures_iter(text){
        println!("match: {:?}",&cap[0]);
    }
}