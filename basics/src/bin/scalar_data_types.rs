fn main(){
    let mut x:f32 = 15.0;
    println!("The value of variable x= {}",x);
    x = 90.0;

    println!("The value of variable x= {}",x);

    //-----------------------------------
    // Scalar Types
    // - Integers 
    //      - Signed ( +, - ) 
    //          - i8, i16, i32, i64
    //      - UnSigned ( + ) 
    //          - u8, u16, u32, u64
    //------------------------------------

    println!("The minimum number in i8 = {}",std::i8::MIN);
    println!("The maximum number in i8 = {}",std::i8::MAX);

    println!("The minimum number in u8 = {}",std::u8::MIN);
    println!("The maximum number in u8 = {}",std::u8::MAX);

    //-------------------------------
    //  - Floats
    //      - f32, f64
    //-------------------------------

    let z: f64 = 3.65;

    println!("The minimum number in f64 = {}",std::f64::MIN);
    println!("The maximum number in f64 = {}",std::f64::MAX);

    //--------------------------------
    //  - Boolean
    //      - true, false
    //--------------------------------
    let status:bool = false;

    println!("The value of some of our variables are {:?}",(x,z,status));

    //---------------------------------
    //  - Characters
    //      - Represent single letters
    //      - digit
    //      - emoji's
    //      - unicode scalar values
    //----------------------------------

    let c1:char = 'a';
    let c2:char = '3';
    let c3:char = '+';
    let c4:char ='\u{288A}';
    let c5:char ='\"';

    println!("The value of c1 is {}, c2 is {}, c3 is {}, c4 is {} and c5 is {}",c1, c2, c3, c4, c5);

}