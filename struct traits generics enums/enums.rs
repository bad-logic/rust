
// enum Conveyance{
//     Car, // 0
//     Train, // 1
//     Air // 2
// }

// enum Conveyance{
//     Car = 10, 
//     Train = 20, 
//     Air = 30
// }

enum Conveyance{
    Car(f32), 
    Train(f32), 
    Air(f32)
}


impl Conveyance {
    // fn travel_allowance(&self, miles:f32) -> f32{
    //     match self {
    //         Conveyance::Car => miles * 14.0,
    //         Conveyance::Train => miles * 19.0,
    //         Conveyance::Air => miles * 23.9 
    //     }

    // }

    fn travel_allowance(&self) -> f32{
        match self {
            Conveyance::Car(miles) => miles * 14.0,
            Conveyance::Train(miles) => miles * 19.0,
            Conveyance::Air(miles) => miles * 23.9 
        }

    }
}

fn main(){
    // let participant1:Conveyance = Conveyance::Car;
    // println!("The conveyance value of the participant1 is {}",participant1 as i32);
    // println!("The travel allowance for participant1 is {}", participant1.travel_allowance(10.0));
    // println!("The travel allowance for participant1 is {}", Conveyance::travel_allowance(&participant1,10.0));
    
    let participant1:Conveyance = Conveyance::Car(10.0);
    println!("The travel allowance for participant1 is {}", participant1.travel_allowance());
    println!("The travel allowance for participant1 is {}", Conveyance::travel_allowance(&participant1));

}