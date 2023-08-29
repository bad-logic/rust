//-----------------------------------------
// 
//              Associated Types
// 
// ----------------------------------------

#[derive(Debug)]
struct Kmh{
    value: u32
}

#[derive(Debug)]
struct Km{
    value: u32
}

#[derive(Debug)]
struct Mph{
    value: u32
}

#[derive(Debug)]
struct Miles{
    value: u32
}

// impl Kmh {
//     fn distance_in_three_hours(&self)-> Km{
//         Km {
//             value: self.value * 3,
//         }
//     }
// }

// impl Mph {
//     fn distance_in_three_hours(&self)-> Miles{
//         Miles {
//             value: self.value * 3,
//         }
//     }
// }

trait DistanceInThreeHours{
    type Distance; // associated type

    fn distance_in_three_hours(&self) -> Self::Distance ;
}

impl DistanceInThreeHours for Kmh{
    type Distance = Km;

    fn distance_in_three_hours(&self) -> Self::Distance{
        Self::Distance {
            value: self.value * 3
        }
    }
}

impl DistanceInThreeHours for Mph{
    type Distance = Miles;
    
    fn distance_in_three_hours(&self) -> Self::Distance{
        Self::Distance {
            value: self.value * 3
        }
    }
}

fn main(){

    let speed = Kmh{value:50};
    let distance = speed.distance_in_three_hours();

    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);


    let speed = Mph{value:50};
    let distance = speed.distance_in_three_hours();

    println!("At {:?}, you will travel {:?} in 3 hours", speed, distance);
}