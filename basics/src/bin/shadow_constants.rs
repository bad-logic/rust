fn main(){
    let (first_number, second_number) = (250, 480.22);
    let large_number:i32 = 1_000_000;

    // let over_flow_number:u8 = 256;

    let x:i32 = 255;
    println!("{} in octal is {:o}, hexadecimal is {:X} and binary is {:b}",x,x,x,x);


    // SHADOWING
    let s:i32 = 5;
    let s:i32 = 5*5; // shadows s in line 12

    println!("value of s is {}",s);

    let mut p:i8 = 5;
    let p:i8 = 5*5; // shadows mutable p by immutable p

    println!("value of p is {}",p);

    let q: i32 = 32;
    let q:char = 'a'; // shadows integer type by character type

    println!("the value of q is {}",q);

    let mut r:i32 = 65;
    {
        let r:i32 = 60; // shadows r within this code segment scope only
        println!("inside code segment r = {}",r);
    }
    println!("outside code segment r = {}",r);

    // CONSTANTS
    // we are not allowed to use mut keyword with constants
    // constants are always immutable and stays immutable
    // type of the value must be annotated, compiler will not infer  and add the type
    // declared using const keyword
    // all uppercase

    const MAX_SALARY:u32 = 100_000;
    println!("max salary is {}",MAX_SALARY)
}