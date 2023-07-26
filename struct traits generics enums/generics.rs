
fn squarei32(x:i32) -> i32{
    x*x
}

fn squaref32(x:f32) -> f32{
    x*x
}


// fn square<T: std::ops::Mul<Output = T> + Copy>(x:T)-> T{
//     x*x
// }

fn square<T>(x:T)-> T
where T: std::ops::Mul<Output = T> + Copy{
    x*x
}

struct Point<T,U>{
    x: T,
    y: U
}

// impl<T: std::fmt::Debug,U: std::fmt::Debug> Point<T,U>{
//     fn printing(&self){
//         println!("(x,y) = ({:?},{:?})",self.x,self.y);
//     }
// }

impl<T,U> Point<T,U>
where T: std::fmt::Debug, U:std::fmt::Debug{
    fn printing(&self){
        println!("(x,y) = ({:?},{:?})",self.x,self.y);
    }
}

fn main(){

    println!("The square of 5 is {}",squarei32(5));
    println!("The square of 5.5 is {}",squaref32(5.5));

    println!("The square of 5 is {}",square(5));
    println!("The square of 5.5 is {}",square(5.5));

    let p1: Point<i32,i32> = Point{x:5,y:5};
    let p2: Point<f32,f32> = Point{x:5.89,y:5.23};
    let p3: Point<i32,f32> = Point{x:5,y:5.23};

    p1.printing();
    Point::printing(&p3);
}