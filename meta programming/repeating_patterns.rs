
macro_rules! string_concat{
    // ()=>{
    //     String::new()
    // };
    // ($some_str:expr)=>{
    //     {   
    //         let mut temp_str = String::new();
    //         temp_str.push_str($some_str);
    //         temp_str
    //     }
    // };
    // ($s1:expr,$s2:expr)=>{
    //     {
    //         let mut temp_str = String::new();
    //         temp_str.push_str($s1);
    //         temp_str.push_str($s2);
    //         temp_str
    //     }
    // };
    ($($some_str:expr),*)=>{
        {
            let mut temp_str = String::new();
            $(temp_str.push_str($some_str);)*
            temp_str
        }
    }
}

macro_rules! vec_mac{
    ($($element: expr),*)=>{
        {
            let mut some_vec = Vec::new();
            $(some_vec.push($element);)*
            some_vec
        }
    }
}

fn main(){
    let str_null = string_concat!();
    let str_single = string_concat!("First");
    let str_double = string_concat!("First","second");

    println!("{} {}",str_single,str_double);

    let string_vec = vec_mac!["Bob","Dylan"];
    println!("{:?}",string_vec);

}