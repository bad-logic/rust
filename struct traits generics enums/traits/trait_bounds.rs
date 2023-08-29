//----------------------------------------
// 
//              Trait Bounds
// 
// ----------------------------------------

trait Double{
    fn double(&self) -> Self;
}

impl Double for i32{
    fn double(&self) -> Self{
        self * 2
    }
}

// impl Double for i64{
//     fn double(&self) -> Self{
//         self * 2
//     }
// }

// Telling compiler T is any type that has Double trait implemented
fn quadruple<T: Double>(x:T) -> T{ 
    x.double().double()
}



fn main(){
    println!("quadruple of 5 = {}", quadruple(5_i32));
    println!("quadruple of 5 = {}", quadruple(5_i64));// error
}