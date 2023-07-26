struct Circle{
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32
}

trait GeneralInfo{
    fn area(&self)-> f32{
        println!("i am not implemented");
        0.0
    }
    fn perimeter(&self) -> f32;
}

impl GeneralInfo for Circle{
    fn area(&self) -> f32{
        3.14 * (self.radius * self.radius)
    }
    fn perimeter(&self) -> f32{
        2.0 * 3.14 * self.radius
    }
}

impl GeneralInfo for Rectangle{
    fn area(&self) -> f32{
        self.length * self.width
    }
    fn perimeter(&self) -> f32{
        2.0 * (self.length + self.width)
    }
}

fn main(){

    let sq:Circle = Circle{
        radius: 5 as f32
    };
    let rect:Rectangle = Rectangle{
        length:5 as f32,
        width:6.0
    };

    println!("Area and perimeter of circle are {} and {}",sq.area(),sq.perimeter());
    println!("Area and perimeter of rectangle are {} and {}",rect.area(),rect.perimeter());

}