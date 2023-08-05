//---------------------------------------
//          function types
//---------------------------------------

fn max(x:i32,y:i32) -> i32 {
    if x > y {x} else {y}
}

fn min(x:i32,y:i32) -> i32 {
    if x < y {x} else {y}
}

fn prints_name(name:&str){
    println!("The name is {}",name);
}

fn prints_full_info(f: fn(&str),some_one: &str, age: i32){
    f(some_one);
    println!("The age is {}",age);
}

fn add_one(x:i32)-> i32{
    x+1
}

fn do_twice(f: fn(i32)->i32, arg:i32)->i32{
    f(f(arg))
}

fn main(){

    let mut f = min;
    println!("The minimum of the two values is {}",f(5,4));
    let g = max;
    println!("The maximum of the two values is {}",g(5,4));
    prints_full_info(prints_name, &String::from("Bob"),40);

    let answer: i32 = do_twice(add_one,5);
    println!("The answer is {}",answer);

}