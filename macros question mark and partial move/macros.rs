//--------------------------------------
//      Declarative Macros
//         
//--------------------------------------

/*

macro_rules! macro_name{
    (...) => {...};
    (...) => {...};
    (...) => {...};
}

*/

macro_rules! cust_macro{
    ()=>{1+1};
    (something matches)=>{
        println!("something matches");
    };
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
    ($a: expr, $b: expr, $c: expr) => {
        $a * ($b + $c)
    };
}

fn main(){
    println!("{}",cust_macro!());
    cust_macro!(something matches);

    println!("{}", cust_macro!(2,3));
    println!("{}", cust_macro!(1,2,3));

}