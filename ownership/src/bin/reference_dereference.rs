fn main(){
    let s1 = String::from("Hello");
    let s2 = &s1; // single reference
    let s3 = &s2; // second order reference
    let s4 = &s3; // third order reference

    println!("{} {} {} {}",s1,s2,s3,s4);
    println!("{}",s1 == "Hello".to_string());
    println!("{}",*s2 == "Hello".to_string());
    println!("{}",**s3 == "Hello".to_string());
    println!("{}",***s4 == "Hello".to_string());
}