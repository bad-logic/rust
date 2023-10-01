//----------------------------------------------------------
// 
//  Regexes - Repeatitions and Quantifiers
// 
//  ? -> indicating 0 or 1 time repeatition
//  + -> indicating 1 or more times of repeatition
//  * -> indicating 0 or more times of repeatition
//----------------------------------------------------------

extern crate regex;

use regex::Regex;

fn main(){
    let regex = Regex::new(r"a?aa").unwrap();
    let text = "aa aaa";

    for cap in regex.captures_iter(text){
        println!("regex: a?aa, match: {:?}",&cap[0]);
    }

    println!();

    let regex = Regex::new(r"a+").unwrap();
    let text = "a aa aaa baab bab b";

    for cap in regex.captures_iter(text){
        println!("regex: a+, match: {:?}",&cap[0]);
    }

    println!();

    let regex = Regex::new(r"ab*").unwrap();
    let text = "a ab abbbbbbba";

    for cap in regex.captures_iter(text){
        println!("regex: ab*, match: {:?}",&cap[0]);
    }


    println!();

    let regex = Regex::new(r"\w{3,5}").unwrap();
    let text = "Hello i think you are awesome";

    for cap in regex.captures_iter(text){
        println!("regex:{:?}, match: {:?}",r"\w{3,5}",&cap[0]);
    }

    println!();

    let regex = Regex::new(r"\b\w{3,5}\b").unwrap();
    let text = "Hello i think you are awesome";

    for cap in regex.captures_iter(text){
        println!("regex:{:?}, match: {:?}",r"\b\w{3,5}\b",&cap[0]);
    }

}