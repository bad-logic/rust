//---------------------------------
// 
//          Trait Items
//              - Self
//              - Functions and Methods
//              - Generic Parameters and Associated types
// 
//---------------------------------


// SELF
trait SimpleTrait {
    fn fn_1() -> i32;
    fn fn_2() -> Self;
}

struct SomeType;
struct OtherType;

impl SimpleTrait for SomeType{
    fn fn_1() -> i32 {
        5
    }

    fn fn_2() -> Self {
        SomeType
    }
}

impl SimpleTrait for OtherType{
    fn fn_1() -> i32 {
        4
    }

    fn fn_2() -> Self {
        OtherType
    }
}

// TRAIT FUNCTIONS AND METHODS
// trait bound function do not use self as first parameter
// trait bound method uses self as first parameter
trait Default{
    fn default() -> Self; // trait(bound) function
} 

impl Default for i32{
    fn default()->Self{
        0
    }
}

// trait (bound) methods
trait TraitMethods {
    // fn fn_1(self);
    fn fn_1(self: Self);
    // fn fn_2(&self);
    fn fn_2(self: &Self);
    // fn fn_3(&mut self);
    fn fn_3(self: &mut Self);
}


// GENERIC PARAMETERS AND ASSOCIATED TYPES

// associated types
// trait Add{
//     type Rhs;
//     type Output;
//     fn add(self,rhs:Self::Rhs) -> Self::Output;
// }

// generic types
trait Add<Rhs, Output>{
    fn add(self,rhs:Rhs) -> Output;
}

#[derive(Debug)]
struct Point{
    x: i32,
    y: i32
}

impl Add<Point,Point> for Point {
    // type Rhs = Point;
    // type Output = Point;

    fn add(self,rhs: Point) -> Point{
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Add<i32,Point> for Point {
    // type Rhs = i32;
    // type Output = Point;

    fn add(self,rhs: i32) -> Point{
        Point{
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Add<Point,Line> for Point {
    fn add(self,rhs:Point) -> Line {
        Line {
            start: self,
            end: rhs
        }
    }
}

fn main(){

    let zero: i32 = Default::default();
    println!("{}",zero);
    // let i: i32;
    // i.default(); // error, this is an associated function, not a method 


    let p1 = Point {x:1,y:23};
    let p2 = Point {x:4,y:2};

    let p3:Point = p1.add(p2);
    println!("{:#?}",p3);

    let p3 = Point {x:4,y:5};
    println!("{:#?}",p3.add(5));


    let p1= Point {x:1,y:12};
    let p2 = Point {x:12,y:12};
    // let p4:Line = p1.add(p2);
    println!("{:#?}",<Point as Add<Point, Line>>::add(p1,p2));



}