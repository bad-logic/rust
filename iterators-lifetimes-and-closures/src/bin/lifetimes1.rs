
fn main(){
    
    let int1: i32 = 5;
    {
        let int2: i32 =  10;
        let result = greater(&int1, &int2);
        println!("The larger value is {}",result);
    }

    let s_1: &str = "Hello";
    let v:&str;
    {
        let s_2:String = String::from("World");
        v = some_fn(s_1,s_2.as_str());
        println!("{}",v);

    }

}

// does not know in compile time whether return is borrowed from i or j
// fn greater(i:&i32,j:&i32) -> &i32 { 
//     if i > j {
//         i
//     }else{
//         j
//     }
// }

// solution generic lifetime parameter
fn greater<'a,'b>(i:&'a i32,j:&'a i32) -> &'a i32 { 
    if i > j {
        i
    }else{
        j
    }
}
// does not know in compile time whether return is borrowed from first_str or second_str
// fn some_fn(first_str:&str,second_str:&str) -> &str{ 
//     first_str
// }

// solution generic lifetime parameter
fn some_fn<'a,'b>(first_str:&'a str,second_str:&'b str) -> &'a str{ 
    first_str
}