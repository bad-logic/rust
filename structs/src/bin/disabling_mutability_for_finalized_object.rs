//-----------------------------------------------
// 
// 
//      - Disabling mutability for finalized objects
// 
// 
//-----------------------------------------------

#[derive(Debug,Clone)]
pub struct finalized_config<T>(T);

impl <T> Copy for finalized_config<T> where T: Copy {

}

impl <T> std::ops::Deref for finalized_config<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[derive(Debug)]
struct  Config{
    a: usize,
    b: String
}

impl Config {
    fn new() -> Self{
        Self { a: 0, b: String::from("Hello") }
    }

    fn build(self)->finalized_config<Config>{
        finalized_config(self)
    }
}

fn main(){

    // disabling mutation after finalizing the object

    // solution 1
    let mut data = vec![5,6,9,4,3];// mutable
    data.sort(); // mutated data

    // no more mutation required so making it immutable
    let data = data;


    // solution 2
    let data = {
        let mut data = vec![5,6,9,4,3];
        data.sort();
        data
    };

    // solution 3
    let mut mu_config = Config::new();
    mu_config.a = 6;

    let finalized = mu_config.build();

    let mut finalized_copy = finalized;

    // finalized_copy.a = 99; // error

}