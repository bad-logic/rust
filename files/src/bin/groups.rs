

extern crate regex;

use regex::Regex;

fn main(){
    let regex = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2050-09-28, 2080-06-12";

    for cap in regex.captures_iter(text){
        println!("{} -> Month: {} Day {} Year {}",&cap[0],&cap[2],&cap[3],&cap[1]);
    }

    println!();
}