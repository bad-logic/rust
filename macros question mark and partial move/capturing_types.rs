
macro_rules! input{
    ($t: ty)=>{
        {
            let mut n = String::new();

            std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");
            
            let n: $t = n.trim().parse().expect("invalid input");
            n
        }
    }
}

macro_rules! add_as{
    ($a:expr,$b:expr,$typ:ty)=>{$a as $typ + $b as $typ}
}

macro_rules! some_macro{
    ($var: ident)=>{
        $var = $var + 1;
    }
}

macro_rules! create_function{
    ($fn_name:ident,$input:ident,$type:ty, $type_out: ty)=>{

        fn $fn_name($input:$type)-> $type_out{
            println!("You called {:?}() with the input of {:?}",stringify!($fn_name),stringify!($input));
            $input
        }

    }
}

create_function!(increment,value,i32,i32);

fn main(){

    println!("Please enter a floating point number");
    let some_input = input!(f32);
    println!("input: {}",some_input);

    println!("{}", add_as!(15,2.9,f32));

    let mut x: i32=4;
    some_macro!(x);
    println!("{}",x);
    x = x + 1;
    println!("{}",x);

    // create_function!(increment,value,i32,i32);
    println!("output {}",increment(5));

}