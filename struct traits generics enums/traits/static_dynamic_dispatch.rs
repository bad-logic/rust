//---------------------------------
// 
// 
//      Static vs Dynamic Dispatch
// 
// 
//---------------------------------


trait Print{
    fn print(&self);
}

impl Print for String{
    fn print(&self){
        println!("I got the value of \"{}\"",self);
    }
}

impl Print for i32{
    fn print(&self){
        println!("I got the value of {}",self);
    }
}

// static dispatch
// multiple versions of this function at runtime
fn static_display<T: Print> (x: T){
    x.print();
}

// dynamic dispatch
// one version of this function at runtime
// fn dynamic_display(x: Vec<&dyn Print>){
//     for i in x{
//         i.print();
//     }
// }

fn dynamic_display(x: Vec<Box<dyn Print>>){
    for i in x{
        i.print();
    }
}


fn main(){

    static_display(5);
    static_display("Hello".to_string());

    // Rust performs monomorphization
    // Due to above two statements,
    // Rust creates two different versions/copies of same function that will be used
    // one for string data type and another for i32
    // for instance:
    /*
        fn static_display_i32(x:i32){
            x.print();
        }

        fn static_display_String(x:String){
            x.print();
        }

        fn main(){

            static_display_i32(5);
            static_display_String("Hello".to_string());

        }
    */

    // dynamic_display(vec![&"Hello world".to_string(), &10]);
    dynamic_display(vec![Box::new("Hello world".to_string()), Box::new(10)]);

}